use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let msg_contents = split_content(&msg.content);

        println!("{:?}", msg_contents);

        if msg_contents[0] == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the enviroment!");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

fn split_content(s: &String) -> Vec<&'static str> {
    s.split_whitespace().collect::<Vec<&str>>()
}
