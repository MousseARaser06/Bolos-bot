use serenity::all::Message;
use std::collections::VecDeque;

pub fn fetch_args(message: &Message) -> VecDeque<&str> {
    let mut args = message
        .content
        .split_whitespace()
        .collect::<VecDeque<&str>>();
    args.remove(0);
    args
}

pub fn fetch_command(args: VecDeque<&str>) -> Option<&str> {
    if args.clone().into_iter().count() < 1 {
        None
    } else {
        Some(args.clone().iter().nth(0).unwrap())
    }
}
