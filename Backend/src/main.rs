extern crate dotenv;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::future::IntoFuture;
use serde::{Serialize,Deserialize};
use gemini_rust::{Gemini, Message, Role, Content};
use tokio;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize)]
struct Config {
    prompt: String,
}


#[derive(Serialize)]
struct Response {
    message: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file contents into environment variables
    dotenv().ok();

    // Get the API key from environment variables at runtime
    let api_key = env::var("GEMINI_API_KEY")?;
    let client = Gemini::new(&api_key);
    
    let response = client.generate_content()
        .with_system_prompt("You are a helpful assistant.")
        .with_user_message("Hello, how are you?")
        .execute()
        .await?;
    
    println!("Response: {}", response.text());
    
    Ok(())
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Backend is somehow still alive :DDDDDDDDDDDDD")
}

#[post("/generate")]
async fn generate_content(config: web::Json<Config>) -> impl Responder {
    // Here you would handle the request to generate content
    HttpResponse::Ok().body("Content generation endpoint received");

    web::Json(Response {
        message: format!("Received prompt: {}", config.prompt),
    })
}

