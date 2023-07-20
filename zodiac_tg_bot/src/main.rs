use teloxide::prelude::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let bot = Bot::from_env().auto_send();

    // let bot = Bot::new("").auto_send();

    let your_id = ChatId(704343182);
    bot.send_message(your_id, "Hi!").await?;    
    // `.await` is needed to wait for an async operation
    // `?` propagates possible errors

        Ok(())
}