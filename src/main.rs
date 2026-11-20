#![feature(linked_list_remove)]

mod commands;
mod misc;

use crate::misc::fetch_args;
use serenity::all::{Context, EventHandler, GatewayIntents, Message};
use serenity::{Client, async_trait};
use std::env;
use crate::commands::run_command;

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
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
