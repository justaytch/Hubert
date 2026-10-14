use super::structs::*;
use fluxer_builders::MessagePayload;
use fluxer_core::client::typed_events::DispatchEvent;
use fluxer_core::client::{Client, ClientOptions};
use fluxer_rest::Rest;
use std::process;

pub async fn run() {
    let config = CONFIG.get().unwrap();

    let options = ClientOptions {
        intents: 0,
        wait_for_guilds: false,
        ..Default::default()
    };

    let mut client = Client::new(options);
    let rest: Rest = client.rest.clone();

    client.on_typed(move |event| {
        let rest = rest.clone();
        Box::pin(async move {
            match event {
                DispatchEvent::Ready => {
                    println!("{} is connected to Fluxer!", config.name);
                }

                DispatchEvent::MessageCreate { message, .. } => {
                    if message.content.trim() == "!h" {
                        let payload = MessagePayload::new().content("h").build();
                        if let Err(e) = message.send(&rest, &payload).await {
                            println!("send: {e}");
                        }
                    }
                    if message.content.trim() == "!logout" {
                        let payload = MessagePayload::new().content("Logging out...").build();
                        if let Err(e) = message.send(&rest, &payload).await {
                            println!("send: {e}");
                        }
                        process::exit(0x0);
                    }
                }

                _ => {}
            }
        })
    });

    if let Err(e) = client.login(&config.fluxer.token).await {
        println!("login: {e:?}");
    }
}
