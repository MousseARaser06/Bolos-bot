mod commands;
mod misc;

use crate::commands::run_command;
use env_file_reader::read_file;
use serenity::all::{Context, EventHandler, GatewayIntents, Message};
use serenity::{Client, async_trait};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.clone().content.into_chars().next() == Some('!') {
            if !message.author.bot {
                run_command(message.clone(), ctx).await;
            }
        }
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
