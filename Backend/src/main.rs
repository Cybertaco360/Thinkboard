extern crate dotenv;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
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
    
    println!("API key loaded successfully");
    let client = Arc::new(Gemini::new(&api_key));
    
    println!("Starting server at http://127.0.0.1:8080");
    
    // Start the web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                gemini_client: client.clone(),
            }))
            .service(hello)
            .service(generate_content)
            .service(sign_in)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;
    
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
        .with_system_prompt("You are a helpful AI assistant. Provide clear, concise, and accurate responses.")
        .with_user_message(&config.prompt)
        .execute()
        .await {
            Ok(response) => {
                web::Json(Response {
                    message: response.text(),
                })
            },
            Err(e) => {
                // Handle errors if the API call fails
                web::Json(Response {
                    message: format!("Error generating content: {}", e),
                })
            }
        }
}

#[post("/signin")]
async fn sign_in(credentials: web::Json<SignInRequest>) -> impl Responder {
    // Define the users directory path
    let users_dir = Path::new("users");
    
    // Check if the directory exists
    if !users_dir.exists() {
        return web::Json(SignInResponse {
            success: false,
            message: "Users database not found".to_string(),
            user_id: None,
        });
    }
    
    // Iterate through user directories
    for entry in fs::read_dir(users_dir).unwrap() {
        if let Ok(entry) = entry {
            let user_info_path = entry.path().join("user_info.json");
            
            // Check if user_info.json exists
            if user_info_path.exists() {
                // Read and parse user_info.json
                if let Ok(file_content) = fs::read_to_string(user_info_path) {
                    if let Ok(user_info) = serde_json::from_str::<UserInfo>(&file_content) {
                        // Check if email matches
                        if user_info.email == credentials.email {
                            // Check password (in a real system, you'd use password hashing)
                            if user_info.password == credentials.password {
                                return web::Json(SignInResponse {
                                    success: true,
                                    message: "Authentication successful".to_string(),
                                    user_id: Some(user_info.id),
                                });
                            } else {
                                return web::Json(SignInResponse {
                                    success: false,
                                    message: "Incorrect password".to_string(),
                                    user_id: None,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    
    // If no matching email was found
    web::Json(SignInResponse {
        success: false,
        message: "User not found".to_string(),
        user_id: None,
    })
}

