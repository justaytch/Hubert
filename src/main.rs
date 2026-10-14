mod commands;
mod discordbot;
mod fluxerbot;
mod stoatbot;
mod structs;

use futures::future;
use std::pin::Pin;
use structs::*;

#[tokio::main]
async fn main() {
    get_config();
    let config = CONFIG.get().unwrap();
    let mut bots: Vec<Pin<Box<dyn Future<Output = ()> + Send>>> = Vec::new();

    if config.discord.enabled {
        bots.push(Box::pin(discordbot::run()));
    }
    if config.fluxer.enabled {
        bots.push(Box::pin(fluxerbot::run()));
    }
    if config.stoat.enabled {
        bots.push(Box::pin(stoatbot::run()));
    }
    if bots.is_empty() {
        println!("Nothing is enabled. Please check config.");
        return;
    }

    future::join_all(bots).await;
}
