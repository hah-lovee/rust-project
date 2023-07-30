use teloxide::{types::Message, Bot};

use teloxide::types::Me;
use teloxide::utils::command::BotCommands;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use dotenv::dotenv;


type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;


const QUESTIONS: &[&'static str] = &[
    "Question 1: What is 2 + 2?",
    "Question 2: What is the capital of France?",
    "Question 3: enter a number from 1 to 5",
    "Question 4: enter a number from 1 to 5",
    "Question 5: enter a number from 1 to 5",
    "Question 6: enter a number from 1 to 5",
    "Question 7: enter a number from 1 to 5",
    "Question 8: enter a number from 1 to 5",
    "Question 9: enter a number from 1 to 5",
    "Question 10: enter a number from 1 to 5",

];

static mut CURRENT_INDEX: usize = 0;

#[derive(Debug)]
pub enum TaskCommand {
    Enable,
    Disable,
    Delete,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "выводит приведственные слова")]
    Start,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Start the test.")]
    Test,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    Test,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot = Bot::from_env();

    println!("Starting...");

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(command_handler))
        .branch(dptree::case![State::Test].endpoint(test_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}


async fn test_handler(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {

    println!("got answer: {}", msg.text().unwrap());
    // Сделать обработку ответа
    if let Some(text) = msg.text() {

        bot.send_message(msg.chat.id, "засчитано!".to_string()).await.unwrap();
    }
    
    unsafe {
        // Здесь в бдшку запрос на отправку должен быть
        if CURRENT_INDEX + 1 >= QUESTIONS.len() {
            bot.send_message(msg.chat.id, "тест окончен".to_string()).await.unwrap();
            CURRENT_INDEX = 0;
            dialogue.update(State::Start).await?;

        } else {

            CURRENT_INDEX += 1;
            bot.send_message(msg.chat.id, QUESTIONS[CURRENT_INDEX].to_string())
                    .await
                    .unwrap();
        }
    

    }
    Ok(())
}

async fn command_handler(bot: Bot, msg: Message, dialogue: MyDialogue, me: Me) -> HandlerResult {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }

            Ok(Command::Test) => {
                dialogue.update(State::Test).await?;
                let message = "Убедительная просьба отвечать только числами от 1 до 5.";
                bot.send_message(msg.chat.id, message.to_string()).await?;
                unsafe { 
                    bot.send_message(msg.chat.id, QUESTIONS[CURRENT_INDEX].to_string())
                    .await
                    .unwrap();
                }

            }

            Ok(Command::Start) => {
                let message = "start";
                bot.send_message(msg.chat.id, message.to_string()).await?;
                // dialogue.update(State::Test).await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
    }

    Ok(())
}