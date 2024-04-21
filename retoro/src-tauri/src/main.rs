// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod chat;
mod config;
mod error;
mod message;
mod network;
mod profile;
mod retoro;
mod utils;

use config::Config;
use profile::Profile;
use retoro::Retoro;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let config = Config::new_from_file("./config.toml")?;
    let profile = Profile::load_from_config(&config)?;

    let mut retoro = Retoro::new(config, profile).await?;
    retoro.run().await?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running Retoro's tauri UI");

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String{
    format!("Hello, {name}!")
}