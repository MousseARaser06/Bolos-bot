#![feature(linked_list_remove)]

mod commands;
mod misc;

use crate::commands::run_command;
use crate::misc::fetch_args;
use env_file_reader::read_file;
use serenity::all::{Context, EventHandler, GatewayIntents, Message};
use serenity::{Client, async_trait};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        let args = fetch_args(&message);
        run_command(message.clone(), args, ctx).await;
    }
}

#[tokio::main]
async fn main() {
    let env_file =
        read_file(".env").expect("Failed to read .env file. Make sure it is present in your CWD.");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&env_file["DISCORD_TOKEN"], intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
