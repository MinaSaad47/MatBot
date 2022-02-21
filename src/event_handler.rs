use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    #[allow(unused_variables)]
    async fn message(&self, ctx: Context, msg: Message) {

    }

    async fn ready(&self, _: Context, rdy: Ready) {
        println!("{} is connected", rdy.user.name);
    }
}


