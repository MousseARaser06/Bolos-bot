use serenity::all::{Context, Message};
use crate::misc::{fetch_args, fetch_command};

mod soundboard;

pub async fn run_command(message: Message, ctx: Context) {
    let args = fetch_args(&message);
    let command = fetch_command(args.clone());

    println!("command : {:?}", command);
    println!("args : {:?}", args);
    match command {
        Some("soundboard") => {
            soundboard::run(&message, args, ctx).await;
        }
        None | Some("help") => {   message.channel_id.say(&ctx.http, "Help message").await.unwrap(); }
        _ => {
            message.channel_id.say(&ctx.http, "Commande invalide").await.unwrap();
        }
    }
}
