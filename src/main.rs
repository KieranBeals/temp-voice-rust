use std::env;
use std::sync::Arc;

use crate::storage::{SQLiteStorage, Storage};
use serenity::all::{EventHandler, GatewayIntents};
use serenity::prelude::TypeMapKey;
use serenity::Client;

mod creator_channel;
mod event_handler;
mod storage;
mod temporary_channel;

pub(crate) struct StorageKey;

impl TypeMapKey for StorageKey {
    type Value = Arc<dyn Storage + Send + Sync>;
}

#[tokio::main]
async fn main() {
    println!("Starting up");

    let database_path = {
        let mut database_path = env::var("DATABASE_PATH").unwrap_or("".to_string());

        if database_path.is_empty() || database_path.ends_with("/") {
            database_path.push_str("database.db");
        }
        
        database_path
    };

    let storage: Arc<dyn Storage + Send + Sync> =
        Arc::new(SQLiteStorage::new(database_path.as_str()).expect("Failed to initialize storage"));

    let mut client: Client = setup_discord_bot().await;

    let mut data = client.data.write().await;
    data.insert::<StorageKey>(Arc::clone(&storage));
    drop(data);

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

async fn setup_discord_bot() -> Client {
    let token = env::var("DISCORD_TOKEN").expect("Expected token: `DISCORD_TOKEN` in the environment");
    
    let intents = GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_PRESENCES;

    Client::builder(&token, intents)
        .event_handler(event_handler::Handler::new())
        .await
        .expect("Err creating client")
}
