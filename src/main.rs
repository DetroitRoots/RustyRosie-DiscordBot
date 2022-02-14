extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
                  
const TOKEN : &str = "";
       //Add your token. Read the documentation for details.
struct Handler;

impl EventHandler for Handler 
{
        if msg.content == "?wru"
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, "This is Rosie, long ago I was an amazing robot in The Jetsons Family. Now I got Rusty and mostly forgotten. I work as an Admin Assistant in this Discord Channel. Let's hug this old rusty machine.")
            {
                println!("Error giving message: {:?}", why);
            }
        }
        {
            if msg.content == "?ping"
            {
                if let Err(why) = msg.channel_id.say(&ctx.http, "pong?")
                {
                    println!("Error giving message: {:?}", why);
                }
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
