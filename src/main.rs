extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
                  
const TOKEN : &str = "";
       //If you want to use this bot, you just need to add your token and compile it, please read the documentation for details.
struct Handler;

impl EventHandler for Handler 
{
     fn message(&self, ctx: Context, msg: Message)
    {
        if msg.content == "?ping"
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong?")
            {
                println!("Error giving message: {:?}", why);
            }
        }
    }
    fn ready(&self, _: Context, ready: Ready) 
    {
        println!("{} is ready", ready.user.name);
    }
}

fn main()
{
    let mut client = Client::new(&TOKEN, Handler)
                        .expect("Error creating client");
    
    if let  Err(msg) = client.start()
   {
        println!("Error: {:?}", msg);
   }
}
