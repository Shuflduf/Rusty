use std::collections::HashMap;
use std::env;
use std::sync::{Arc, OnceLock};

use gemini_rs::safety::safety_settings_from;
use gemini_rs::Conversation;
use serenity::all::{ChannelId, Http, Message, Ready};
use serenity::async_trait;
use serenity::prelude::*;
use lazy_static::lazy_static;

static BOT_ID: OnceLock<u64> = OnceLock::new();
lazy_static! {
    static ref SERVER_HISTORIES: Mutex<HashMap<ChannelId, Conversation>> = Mutex::new(HashMap::new());
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        let _ = BOT_ID.get_or_init(|| ready.user.id.get());
    }
    async fn message(&self, ctx: Context, msg: Message) {
        let bot_id = *BOT_ID.get().unwrap();
        if bot_id != 0 && msg.author.id.get() == bot_id {
            return;
        }

        if msg.content == "$hello" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hello!").await {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content == "$convo" {
                if SERVER_HISTORIES.lock().await
                .contains_key(&msg.channel_id)
            {
                let mut histories = SERVER_HISTORIES.lock().await;
                histories.remove(&msg.channel_id);
            } else {
                let mut histories = SERVER_HISTORIES.lock().await;
                histories.insert(msg.channel_id, Conversation::new(
                    env::var("GEMINI_API_KEY").unwrap(),
                    "gemini-1.5-flash".to_string()
                ));
            }

        } else if 
            SERVER_HISTORIES.lock().await
            .contains_key(&msg.channel_id)
        {
            let mut histories = SERVER_HISTORIES.lock().await;
            if let Some(convo) = histories.get_mut(&msg.channel_id) {
                send_ai_message(&ctx, &msg, &msg.content, Some(convo)).await
            }
        } else if msg.content.starts_with("$") {
            send_ai_message(&ctx, &msg, &msg.content[1..], None).await
        } 
    }
}

async fn send_long_message(channel: ChannelId, text: &str, ctx: &Context) {
    let mut index = 0;
    while index < text.len() {
        let mut part = text[index..].to_string();
        part.truncate(2000);
        println!("{part}");
        let _ = channel.say(ctx.http.clone(), part).await;
        index += 2000;
    };
}

async fn send_ai_message(ctx: &Context, msg: &Message, text: &str, convo: Option<&mut Conversation>) {
    let conversation = match convo {
        None => { 
            let api_key = env::var("GEMINI_API_KEY").unwrap();
            let mut c = Conversation::new(
                api_key,
                "gemini-1.5-flash".to_string()
            );
            c.update_safety_settings(safety_settings_from(gemini_rs::safety::HarmBlockThreshold::Off));
            c
        },
        Some(cool_its_done) => cool_its_done,
    };
    let http = &Arc::new(Http::new(&env::var("RUSTY_TOKEN").unwrap()));
    let typing = msg.channel_id.start_typing(http);
    let response = conversation.prompt(text).await;

    send_long_message(msg.channel_id, &response, ctx).await;
    typing.stop();
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("RUSTY_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
