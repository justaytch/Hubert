mod commands;

use regex::Regex;
use reqwest::Client as RestClient;
use serde::Serialize;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId as DiscordUserId;
use serenity::prelude::*;
use std::sync::LazyLock;

static TOKEN: &str = "token";
static FLUXERWEBHOOK: &str = "fluxer webhook link";
//put the channel id here
static CHANNELID: u64 = 69420;
static DEFAULTAVATAR: &str = "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRbLisRGhHa5jC33v1nspPUrVcBNe44ZfeJe57chkvzmpM3U2GdPT4RnLy2&s=10";

static RESTCLIENT: LazyLock<RestClient> = LazyLock::new(RestClient::new);

#[derive(Serialize)]
struct WebhookPost {
    username: String,
    content: String,
    avatar_url: String,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "h" => Some(commands::h::run(&command.data.options())),
                _ => Some("h".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot {
            if msg.channel_id.get() == CHANNELID {
                let re = Regex::new(r"[<][@]!?(\d{17,19})[>]").unwrap();
                let mut msgnew = msg.content.clone();
                for caps in re.captures_iter(&msg.content) {
                    let id: u64 = caps[1].parse().expect("Id is Not a valid number");
                    let user = DiscordUserId::new(id)
                        .to_user(&ctx.http)
                        .await
                        .expect("Error Parsing Id");
                    msgnew = msgnew.replace(
                        &caps[0],
                        &format!(
                            "@{}",
                            &user
                                .nick_in(&ctx.http, &msg.guild_id.unwrap_or_default())
                                .await
                                .as_deref()
                                .unwrap_or("Error")
                        ),
                    );
                }
                let payload = WebhookPost {
                    content: String::from(&msgnew),
                    username: String::from(format!(
                        "{} (Discord User)",
                        msg.author_nick(&ctx.http).await.as_deref().unwrap_or(
                            msg.author
                                .global_name
                                .as_deref()
                                .unwrap_or(&msg.author.name),
                        )
                    )),
                    avatar_url: String::from(
                        msg.author.avatar_url().as_deref().unwrap_or(DEFAULTAVATAR),
                    ),
                };
                let _ = RESTCLIENT.post(FLUXERWEBHOOK).json(&payload).send().await;
            }
            if msg.content == "!h" {
                if let Err(why) = msg.channel_id.say(&ctx.http, "h").await {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected to Discord!", ready.user.name);

        if let Err(why) = Command::set_global_commands(
            &ctx.http,
            vec![commands::ping::register(), commands::h::register()],
        )
        .await
        {
            println!("Could not register commands: {why:?}")
        };
    }
}

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(TOKEN, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
