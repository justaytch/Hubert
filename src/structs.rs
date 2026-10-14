use reqwest::Client as RestClient;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::sync::OnceLock;

#[derive(Deserialize)]
pub struct Config {
    pub botname: String,
    pub discordtoken: String,
    pub fluxertoken: String,
    pub fluxerwebhook: String,
    pub stoatwebhook: String,
    pub discordchannelid: u64,
    pub discordownerids: Vec<u64>,
    pub defaultavatar: String,
    pub fluxer: bool,
    pub stoat: bool,
    pub discord: bool,
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
        let json_str = include_str!("./config.json");
        serde_json::from_str(json_str).expect("Invalid JSON syntax in config.json")
    })
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();
pub static RESTCLIENT: LazyLock<RestClient> = LazyLock::new(RestClient::new);
