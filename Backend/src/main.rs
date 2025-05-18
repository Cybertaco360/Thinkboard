extern crate dotenv;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header;
use actix_cors::Cors;
use std::collections::HashMap;
use std::future::IntoFuture;
use serde::{Serialize,Deserialize};
use gemini_rust::{Gemini, Message, Role, Content};
use tokio;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use std::path::Path;
use std::fs;
use mongodb::bson::doc;
use mongodb::bson::DateTime;


#[derive(Deserialize)]
struct Config {
    prompt: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

#[derive(Deserialize)]
struct SignInRequest {
    email: String,
    password: String,  // Note: In production, never store passwords in plain text!
}

#[derive(Serialize)]
struct SignInResponse {
    success: bool,
    message: String,
    user_id: Option<String>,
}

#[derive(Deserialize)]
struct SignUpRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    id: String,
    name: String,
    email: String,
    created_at: String,
    last_login: String,
    password: String,  // Adding password field to match our implementation
}

struct AppState {
    gemini_client: Arc<Gemini>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file contents into environment variables
    // Print whether the .env file was loaded successfully
    if dotenv().is_ok() {
        println!("Successfully loaded .env file");
    } else {
        println!("Warning: .env file not found or could not be loaded");
    }

    // Get the API key from environment variables at runtime
    let api_key = match env::var("GEMINI_API_KEY") {
        Ok(key) => {
            if key.is_empty() {
                return Err("GEMINI_API_KEY environment variable is empty".into());
            }
            key
        },
        Err(_) => return Err("GEMINI_API_KEY environment variable not set. Please create a .env file with your API key".into()),
    };

    let database_key = match env::var("MONGODB_URI") {
        Ok(key) => {
            if key.is_empty() {
                return Err("MONGODB_URI environment variable is empty".into());
            }
            key
        },
        Err(_) => return Err("MONGODB_URI environment variable not set. Please create a .env file with your API key".into()),
    };
    
    println!("API key loaded successfully");
    let client = Arc::new(Gemini::new(&api_key));
    
    println!("Starting server at http://127.0.0.1:8080");
    
    // Start the web server
    HttpServer::new(move || {
let cors = Cors::default()
    .allowed_origin("http://localhost:5173")
    .allowed_methods(vec!["GET", "POST"])
    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
    .allowed_header(header::CONTENT_TYPE)
    .max_age(3600);

        App::new()
        .wrap(cors)
            .app_data(web::Data::new(AppState {
                gemini_client: client.clone(),
            }))
            .service(hello)
            .service(generate_content)
            .service(sign_in)
            .service(sign_up)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    
    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Backend is somehow still alive :DDDDDDDDDDDDD")
}

#[post("/generate")]
async fn generate_content(config: web::Json<Config>, data: web::Data<AppState>) -> impl Responder {
    let client = &data.gemini_client;

    match client.generate_content()
        .with_system_prompt(r#"You are a helpful AI assistant. Provide your response as a single JSON array of nodes. Each node must use this schema: { node_id: #, x: X-COORDINATE, y: Y-COORDINATE, text: "TEXT THAT WILL BE DISPLAYED ON THE NODE", connected: [OTHER NODES TO BE CONNECTED TO], information: "Information at this certain point" }. Do not put or return in a codeblock. Make sure that there's no ```json ``` or anything like that. Do not return anything except the JSON array. Each Node has a width of 270px and a height of 100px, the X and Y you are going to be providing is always going to be in the unit PX"#)
        .with_user_message(&config.prompt)
        .execute()
        .await {
            Ok(response) => {
                let mut text = response.text();
                // Try to parse as JSON array first
                let try_parse = serde_json::from_str::<serde_json::Value>(&text);
                if try_parse.is_ok() {
                    return web::Json(try_parse.unwrap());
                }
                // If not, try to wrap in brackets and parse as array
                if text.trim().starts_with('{') && text.trim().ends_with('}') {
                    text = format!("[{}]", text);
                }
                match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(json) => web::Json(json),
                    Err(_) => {
                        println!("{}", response.text());
                        web::Json(serde_json::json!({ "error": "Invalid schema from Gemini" }))
                    }
                }
            },
            Err(e) => {
                web::Json(serde_json::json!({ "error": format!("API call failed: {}", e) }))
            }
        }
}

#[post("/login")]
async fn sign_in(credentials: web::Json<SignInRequest>, data: web::Data<AppState>) -> impl Responder {
    // Get MongoDB connection string from environment
    let mongodb_uri = match env::var("MONGODB_URI") {
        Ok(uri) => uri,
        Err(_) => {
            return web::Json(SignInResponse {
                success: false,
                message: "Database configuration error".to_string(),
                user_id: None,
            });
        }
    };

    // Connect to MongoDB
    let client_options = match mongodb::options::ClientOptions::parse(&mongodb_uri).await {
        Ok(opts) => opts,
        Err(e) => {
            return web::Json(SignInResponse {
                success: false,
                message: format!("Failed to parse MongoDB connection string: {}", e),
                user_id: None,
            });
        }
    };

    let client = match mongodb::Client::with_options(client_options) {
        Ok(client) => client,
        Err(e) => {
            return web::Json(SignInResponse {
                success: false,
                message: format!("Failed to connect to MongoDB: {}", e),
                user_id: None,
            });
        }
    };

    // Access database and collection
    let database = client.database("jamhacks");
    let users_collection = database.collection::<UserInfo>("users");

    // Find user by email
    let filter = doc!{ "email": &credentials.email };
    
    match users_collection.find_one(filter).await {
        Ok(Some(user)) => {
            // Check password (in a real system, you'd use password hashing)
            if user.password == credentials.password {
                web::Json(SignInResponse {
                    success: true,
                    message: "Authentication successful".to_string(),
                    user_id: Some(user.id),
                })
            } else {
                web::Json(SignInResponse {
                    success: false,
                    message: "Incorrect password".to_string(),
                    user_id: None,
                })
            }
        },
        Ok(None) => {
            web::Json(SignInResponse {
                success: false,
                message: "User not found".to_string(),
                user_id: None,
            })
        },
        Err(e) => {
            web::Json(SignInResponse {
                success: false,
                message: format!("Database error: {}", e),
                user_id: None,
            })
        }
    }
}

#[post("/signup")]
async fn sign_up(credentials: web::Json<SignUpRequest>) -> impl Responder {
    use chrono::Utc;

    // Get MongoDB connection string from environment
    let mongodb_uri = match env::var("MONGODB_URI") {
        Ok(uri) => uri,
        Err(_) => {
            return web::Json(SignInResponse {
                success: false,
                message: "Database configuration error".to_string(),
                user_id: None,
            });
        }
    };

    // Connect to MongoDB
    let client_options = match mongodb::options::ClientOptions::parse(&mongodb_uri).await {
        Ok(opts) => opts,
        Err(e) => {
            return web::Json(SignInResponse {
                success: false,
                message: format!("Failed to parse MongoDB connection string: {}", e),
                user_id: None,
            });
        }
    };

    let client = match mongodb::Client::with_options(client_options) {
        Ok(client) => client,
        Err(e) => {
            return web::Json(SignInResponse {
                success: false,
                message: format!("Failed to connect to MongoDB: {}", e),
                user_id: None,
            });
        }
    };

    let database = client.database("jamhacks");
    let users_collection = database.collection::<UserInfo>("users");

    // Check if user already exists
    let filter = doc! { "email": &credentials.email };
    match users_collection.find_one(filter.clone()).await {
        Ok(Some(_)) => {
            return web::Json(SignInResponse {
                success: false,
                message: "User already exists".to_string(),
                user_id: None,
            });
        }
        Err(e) => {
            return web::Json(SignInResponse {
                success: false,
                message: format!("Database error: {}", e),
                user_id: None,
            });
        }
        _ => {}
    }

    // Create new user
    let user_id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let new_user = UserInfo {
        id: user_id.clone(),
        name: credentials.name.clone(),
        email: credentials.email.clone(),
        created_at: now.clone(),
        last_login: now,
        password: credentials.password.clone(), // In production, hash this!
    };

    match users_collection.insert_one(&new_user).await {
        Ok(_) => web::Json(SignInResponse {
            success: true,
            message: "User registered successfully".to_string(),
            user_id: Some(user_id),
        }),
        Err(e) => web::Json(SignInResponse {
            success: false,
            message: format!("Failed to register user: {}", e),
            user_id: None,
        }),
    }
}

