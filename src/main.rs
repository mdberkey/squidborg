use std::env;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping, ask)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

use tokio::runtime::Runtime;

#[command]
async fn ask(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "thinking... ").await?;
    let api_token = std::env::var("OPENAI_SK").unwrap();
    let client = openai_api::Client::new(&api_token);
    let prompt = "Answer the following question accurately, 
                              but find a funny way to mention a squid in your response.";
    let rt = Runtime::new().unwrap();
    rt.block_on(
        let response = &client.complete_prompt(prompt).await.unwrap().to_string();
        );
    println!("{} bruh", response);
    msg.reply(ctx, "done.").await?;
    Ok(())
}

