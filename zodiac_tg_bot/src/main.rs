use teloxide::{types::Message, Bot};

use teloxide::types::Me;
use teloxide::utils::command::BotCommands;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use dotenv::dotenv;


type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

struct QuestionsHolder(Vec<&'static str>);

const QUESTIONS: &[&'static str] = &[
    "Question 1: What is 2 + 2?",
    "Question 2: What is the capital of France?",
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
    CheckAnswer,
}

#[tokio::main]
async fn main() {
    dotenv().ok();


    let bot = Bot::from_env();

    println!("Starting...");

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(command_handler))
        .branch(dptree::case![State::Test].endpoint(test_handler))
        .branch(dptree::case![State::CheckAnswer].endpoint(check_answer))
        ;

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        // .dependencies(dptree::deps![InMemStorage::<Store>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn test_handler(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    println!("Entered test_handler");
    unsafe {
    println!("asking question: {}", QUESTIONS[CURRENT_INDEX]);

    bot.send_message(msg.chat.id, QUESTIONS[CURRENT_INDEX].to_string())
        .await
        .unwrap();
        
    }
    dialogue.update(State::CheckAnswer).await?;

    
    
    Ok(())
}

async fn check_answer(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {

    println!("got answer: {}", msg.text().unwrap());
    if let Some(text) = msg.text() {

        bot.send_message(msg.chat.id, "засчитано!".to_string()).await;
    }
    
    unsafe {
        if CURRENT_INDEX + 1 >= QUESTIONS.len() {
            bot.send_message(msg.chat.id, "тест окончен".to_string()).await;
            dialogue.update(State::Start).await?;

        } else {
            dialogue.update(State::Test).await?;
            CURRENT_INDEX += 1
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

                bot.send_message(msg.chat.id, "введите ваш пол (м/ж)".to_string()).await?;
            }

            Ok(Command::Start) => {
                bot.send_message(msg.chat.id, "start".to_string()).await?;
                // dialogue.update(State::Test).await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
    }

    Ok(())
}