
use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;


fn get_weather() -> String {
    return "69* celcjusza".to_string();
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            println!("sent by bot!");
            return;
        }
        println!("author : {}, content: {}, tts: {}", msg.author.display_name().to_string(), msg.content, msg.tts);
        match msg.content.as_str(){
            "!ping" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    println!("Error sending message: {why:?}");
                }
            }
            "!weather" => {
                let weather = get_weather();
                if let Err(why) = msg.channel_id.say(&ctx.http, format!("weather is {}", weather)).await {
                    println!("Error sending message: {why:?}");
                }
            }
            _ => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "unkown command").await {
                    println!("Error sending message: {why:?}");
                }
            }
        }

    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
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