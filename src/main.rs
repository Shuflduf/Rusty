use std::env;

use serenity::all::Ready;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref BOT_ID: u64 = 0;
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        //*BOT_ID = ready.user.id.get()
        #![warn(unused_assignments)]
        let mut bot_id = *BOT_ID;

        bot_id = ready.user.id.get();
    }
    async fn message(&self, ctx: Context, msg: Message) {
        let bot_id = &BOT_ID;

        if **bot_id != 0 && msg.author.id.get() == **bot_id {
            return;
        }

        if msg.content == "$hello" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hello!").await {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content.starts_with("$") {
            let key = env::var("GEMINI_API_KEY").unwrap();
            let mut response = gemini_rs::Conversation::new(
                key,
                "gemini-1.5-flash".to_string()
            ).prompt(&msg.content[1..]).await;
            response.truncate(2000);

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
