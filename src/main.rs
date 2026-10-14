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

    if config.discord {
        bots.push(Box::pin(discordbot::run()));
    }
    if config.fluxer {
        bots.push(Box::pin(fluxerbot::run()));
    }
    if config.stoat {
        bots.push(Box::pin(stoatbot::run()));
    }

    future::join_all(bots).await;
}
