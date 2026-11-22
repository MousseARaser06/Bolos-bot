use serenity::all::{Context, CreateMessage, Message};
use serenity::builder::CreateEmbed;
use std::collections::VecDeque;

pub async fn help_subcommand(message: &Message, ctx: Context) {
    let embed = CreateEmbed::new().title("Menu d'aide : /arbre soundboard").description("/arbre soundboard play <?>
    /arbre soundboard list");
    message.channel_id.send_message(&ctx.http, CreateMessage::new().embed(embed)).await.ok();
}

pub async fn play_subcommand(message: &Message, ctx: Context) {

}

pub async fn list_subcommand(message: &Message, ctx: Context) {
    message.channel_id.say(&ctx.http, "Liste :)").await.unwrap();
}

pub async fn run(message: &Message, args: VecDeque<&str>, ctx: Context) {
        match args.get(0).unwrap_or_else(|| &"help") {
            &"list" => list_subcommand(message, ctx).await,
            &"play" => play_subcommand(message, ctx).await,
            &"help" => help_subcommand(message, ctx).await,
            _ => {
                list_subcommand(message, ctx).await;
            }
        }
}
