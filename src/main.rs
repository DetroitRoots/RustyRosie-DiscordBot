extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
                  
const TOKEN : &str = "";
      //ボットを作成した後、Discord開発者ページからトークンを取得する必要があります。
struct Handler;

impl EventHandler for Handler 
{
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
