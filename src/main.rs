use std::collections::HashMap;
use std::env;
use std::sync::{Arc, OnceLock};

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
                // If the channel is in convo mode
                let mut histories = SERVER_HISTORIES.lock().await;
                histories.remove(&msg.channel_id)
                

            } else {
                // Add the channel to the list 
                let mut histories = SERVER_HISTORIES.lock().await;
                histories.insert(msg.channel_id, Conversation::new(
                    env::var("GEMINI_API_KEY").unwrap(),
                    "gemini-1.5-flash".to_string()
                ))
            }

        } else if 
            SERVER_HISTORIES.lock().await
            .contains_key(&msg.channel_id)
        {
            let mut histories = SERVER_HISTORIES.lock().await;
            if let Some(convo) = histories.get_mut(&msg.channel_id) {
                todo!()
            }
            //let mut convo: &mut Conversation = histories.get(&msg.channel_id).unwrap();
            //println!("{0:?}", *convo.prompt("a").await);
        } else if msg.content.starts_with("$") {
            send_message(&ctx, &msg, &msg.content[1..], None).await
            //let key = env::var("GEMINI_API_KEY").unwrap();
            //let http = &Arc::new(Http::new(&env::var("RUSTY_TOKEN").unwrap()));
            //let typing = msg.channel_id.start_typing(http);
            //let mut response = gemini_rs::Conversation::new(
            //    key,
            //    "gemini-1.5-flash".to_string()
            //).prompt(&msg.content[1..]).await;
            //response.truncate(2000);
            //
            //typing.stop();
            //if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            //    println!("Error sending message: {why:?}");
            //}
        } 
    }
}

async fn send_message(ctx: &Context, msg: &Message, text: &str, convo: Option<Conversation>) {
    let mut conversation = match convo {
        None => Conversation::new(
                env::var("GEMINI_API_KEY").unwrap(),
                "gemini-1.5-flash".to_string()
            ),
        Some(cool_its_done) => cool_its_done,
    };
    let http = &Arc::new(Http::new(&env::var("RUSTY_TOKEN").unwrap()));
    let typing = msg.channel_id.start_typing(http);
    let mut response = conversation.prompt(text).await;
    response.truncate(2000);

    typing.stop();
    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
        println!("Error sending message: {why:?}");
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
