use std::collections::LinkedList;
use serenity::all::Message;

pub fn fetch_args(message: &Message) -> LinkedList<&str> {
    let mut args = message.content.split_whitespace().collect::<LinkedList<&str>>();
    args.remove(1);
    args
}