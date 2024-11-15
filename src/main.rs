use std::collections::HashMap;
use std::env;
use std::sync::{Arc, OnceLock};

use serenity::all::{Http, Ready};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

static BOT_ID: OnceLock<u64> = OnceLock::new();
//static SERVER_HISTORIES: HashMap<ChannelFlags>

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
        } else if msg.content.starts_with("$") {
            let key = env::var("GEMINI_API_KEY").unwrap();
            let http = &Arc::new(Http::new(&env::var("RUSTY_TOKEN").unwrap()));
            let typing = msg.channel_id.start_typing(http);
            let mut response = gemini_rs::Conversation::new(
                key,
                "gemini-1.5-flash".to_string()
            ).prompt(&msg.content[1..]).await;
            response.truncate(2000);

            typing.stop();
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending message: {why:?}");
            }
        }
    }
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
