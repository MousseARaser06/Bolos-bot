use serenity::all::Message;
use std::collections::LinkedList;

pub fn fetch_args(message: &Message) -> LinkedList<&str> {
    let mut args = message
        .content
        .split_whitespace()
        .collect::<LinkedList<&str>>();
    args.remove(1);
    args
}
