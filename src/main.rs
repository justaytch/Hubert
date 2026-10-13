mod commands;
mod discordbot;
mod fluxerbot;
mod stoatbot;
mod structs;

use structs::*;

#[tokio::main]
async fn main() {
    get_config();
    let config = CONFIG.get().unwrap();
    if config.discord && !config.fluxer && !config.stoat {
        discordbot::run();
    } else if config.fluxer && !config.discord && !config.stoat {
        fluxerbot::run();
    } else if config.stoat && !config.discord && !config.fluxer {
        stoatbot::run();
    } else if config.discord && config.fluxer && !config.stoat {
        let discord = std::thread::spawn(discordbot::run);
        let fluxer = std::thread::spawn(fluxerbot::run);
        discord.join().expect("Discord bot panicked");
        fluxer.join().expect("Fluxer bot panicked");
    } else if config.discord && config.stoat && !config.fluxer {
        let discord = std::thread::spawn(discordbot::run);
        let stoat = std::thread::spawn(stoatbot::run);
        discord.join().expect("Discord bot panicked");
        stoat.join().expect("Stoat bot panicked");
    } else if config.fluxer && config.stoat && !config.discord {
        let fluxer = std::thread::spawn(fluxerbot::run);
        let stoat = std::thread::spawn(stoatbot::run);
        fluxer.join().expect("Fluxer bot panicked");
        stoat.join().expect("Stoat bot panicked");
    } else if config.fluxer && config.stoat && config.discord {
        let fluxer = std::thread::spawn(fluxerbot::run);
        let stoat = std::thread::spawn(stoatbot::run);
        let discord = std::thread::spawn(discordbot::run);
        fluxer.join().expect("Fluxer bot panicked");
        stoat.join().expect("Stoat bot panicked");
        discord.join().expect("Discord bot panicked");
    } else {
        println!("Nothing is enabled. Please check config.")
    }
}
