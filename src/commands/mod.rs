use serenity::all::{Context, Message};
use std::collections::LinkedList;

mod soundboard;

pub enum Commands {
    Soundboard(),
}

pub async fn parse_command(message: Message, ctx: &Context) -> Option<Commands> {
    let command = message.content.clone();
    match command.as_str() {
        "soundboard" => Some(Commands::Soundboard()),
        _ => {
            if let Err(err) = message.channel_id.say(&ctx.http, format!("Commande {} introuvable", command)).await {
                println!("Error sending message: {:?}", err);
            };
            None
        }
    }
}

pub async fn run_command(message: Message, args: LinkedList<&str>, ctx: Context ) {
    let command = parse_command(message, &ctx).await.unwrap();
    match command {
        Commands::Soundboard() => {
            soundboard::run(args, ctx);
        },
    }
}