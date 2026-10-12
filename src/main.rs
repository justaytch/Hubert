mod commands;
mod discord;
mod structs;

use structs::*;

fn main() {
    get_config();
    if CONFIG.get().unwrap().discord {
        discord::run();
    }
    if !CONFIG.get().unwrap().discord
        && !CONFIG.get().unwrap().fluxer
        && !CONFIG.get().unwrap().stoat
    {
        println!("Nothing is enabled. Please check config.")
    }
}
