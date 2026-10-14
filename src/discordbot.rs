use super::commands;
use super::structs::*;

use regex::Regex;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::gateway::ActivityData;
use serenity::model::application::{Command, Interaction};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::id::RoleId;
use serenity::model::id::UserId;
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;
use std::process;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
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
        let config = &CONFIG.get().unwrap();
        if !msg.author.bot {
            if msg.channel_id.get() == config.discord.channelid {
                if config.fluxer.enabled || config.stoat.enabled {
                    let mut re = Regex::new(r"[<][@]!?(\d{17,19})[>]").unwrap();
                    let mut msgnew = msg.content.clone();
                    for caps in re.captures_iter(&msg.content) {
                        let id: u64 = caps[1].parse().expect("Id is Not valid");
                        let user = UserId::new(id)
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
                                    .unwrap_or(
                                        &msg.author
                                            .global_name
                                            .as_deref()
                                            .unwrap_or(&msg.author.name)
                                    )
                            ),
                        );
                    }
                    re = Regex::new(r"[<][#](\d{17,19})[>]").unwrap();
                    for caps in re.captures_iter(&msg.content) {
                        let id: u64 = caps[1].parse().expect("Id is not valid");
                        let chan = ChannelId::new(id)
                            .to_channel(&ctx.http)
                            .await
                            .expect("Error Parsing Id");
                        msgnew =
                            msgnew.replace(&caps[0], &format!("#{}", &chan.guild().unwrap().name));
                    }
                    re = Regex::new(r"[<][@][&](\d{17,19})[>]").unwrap();
                    for caps in re.captures_iter(&msg.content) {
                        let id: u64 = caps[1].parse().expect("Id is not valid");
                        let guild = msg.guild(&ctx.cache).unwrap();
                        let role = guild.roles.get(&RoleId::new(id)).unwrap();
                        msgnew = msgnew.replace(&caps[0], &format!("@{}", role.name));
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
                            msg.author
                                .avatar_url()
                                .as_deref()
                                .unwrap_or(&config.fluxer.defaultavatar),
                        ),
                    };
                    let stoatpayload = StoatWebhookPost {
                        content: String::from(&msgnew),
                        masquerade: Masquerade {
                            name: String::from(format!(
                                "{} (Discord User)",
                                msg.author_nick(&ctx.http).await.as_deref().unwrap_or(
                                    msg.author
                                        .global_name
                                        .as_deref()
                                        .unwrap_or(&msg.author.name),
                                )
                            )),
                            avatar: String::from(
                                msg.author
                                    .avatar_url()
                                    .as_deref()
                                    .unwrap_or(&config.stoat.defaultavatar),
                            ),
                        },
                    };
                    if config.fluxer.enabled {
                        let _ = RESTCLIENT
                            .post(&config.fluxer.webhook)
                            .json(&payload)
                            .send()
                            .await;
                    }
                    if config.stoat.enabled {
                        let _ = RESTCLIENT
                            .post(&config.stoat.webhook)
                            .json(&stoatpayload)
                            .send()
                            .await;
                    }
                }
            }
            if msg.content == "!h" {
                if let Err(why) = msg.channel_id.say(&ctx.http, "h").await {
                    println!("Error sending message: {why:?}");
                }
            }
            if msg.content == "!logout" && config.discord.ownerids.contains(&msg.author.id.get()) {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Logging Out...").await {
                    println!("Error sending message: {why:?}");
                }
                process::exit(0x0);
            }
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        println!("{} is connected to Discord!", CONFIG.get().unwrap().name);

        if let Err(why) = Command::set_global_commands(
            &ctx.http,
            vec![commands::ping::register(), commands::h::register()],
        )
        .await
        {
            println!("Could not register commands: {why:?}")
        };
        let activity = ActivityData::custom(format!(
            "Watching #{}",
            ChannelId::new(CONFIG.get().unwrap().discord.channelid)
                .name(&ctx.http)
                .await
                .unwrap_or("Unknown".to_string())
        ));
        ctx.set_presence(Some(activity), OnlineStatus::DoNotDisturb);
    }
}

pub async fn run() {
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&CONFIG.get().unwrap().discord.token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
