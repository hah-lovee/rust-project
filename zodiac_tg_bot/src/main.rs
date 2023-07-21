use teloxide::prelude::*;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env().auto_send();

    // let your_id = ChatId(704343182);
    // bot.send_message(your_id, "Hi!").await?;    


    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        // There are non-text messages, so we need to use pattern matching
        if let Some(text) = message.text() {
            // Echo text back into the chat
            println!("{text}");
            bot.send_message(message.chat.id, text).await?;
        }
    
        // respond is an alias to `Ok()` with a error type compatible with teloxide
        respond(())
    }).await;

        Ok(())
}