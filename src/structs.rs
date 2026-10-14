use reqwest::Client as RestClient;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::{LazyLock, OnceLock};

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub discord: BotConfig,
    pub fluxer: BotConfig,
    pub stoat: BotConfig,
}

#[derive(Deserialize, Debug)]
pub struct BotConfig {
    pub enabled: bool,
    pub token: String,
    pub channelid: u64,
    pub ownerids: Vec<u64>,
    pub webhook: String,
    pub defaultavatar: String,
}

#[derive(Serialize)]
pub struct WebhookPost {
    pub username: String,
    pub content: String,
    pub avatar_url: String,
}

#[derive(Serialize)]
pub struct Masquerade {
    pub name: String,
    pub avatar: String,
}

#[derive(Serialize)]
pub struct StoatWebhookPost {
    pub content: String,
    pub masquerade: Masquerade,
}

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let text = fs::read_to_string("./config.toml").expect("Error opening ./config.toml");
        toml::from_str(&text).expect("Invalid TOML syntax in ./config.toml")
    })
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();
pub static RESTCLIENT: LazyLock<RestClient> = LazyLock::new(RestClient::new);
