use std::sync::Mutex;
use std::fs::OpenOptions;


use teloxide::{types::{Message, Me}, Bot};
use teloxide::utils::command::BotCommands;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use dotenv::dotenv;
use itertools::Itertools;
use csv::WriterBuilder;



type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;


const QUESTIONS: &[&'static str] = &[
    "Question 1: What is 2 + 2?",
    "Question 2: enter a number from 1 to 5",
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

static mut GLOBAL_VECTOR: Option<Mutex<Vec<i8>>> = None;


#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "outputs welcome words")]
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

    unsafe {
        GLOBAL_VECTOR = Some(Mutex::new(Vec::new()));
    }

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
    
    if let Some(text) = msg.text() {

        if text.chars().count() > 1 {
            bot.send_message(msg.chat.id, "try digit from 1 to 5.".to_string()).await.unwrap();
        } else {

            if let Some(digit_char) = text.chars().find(|c| c.is_digit(6)) {

                if let Some(digit) = digit_char.to_digit(6) {
                    println!("Found digit: {}", digit);
                    if digit != 0 {
                        let digit = digit as i8;
                        unsafe {

                            if CURRENT_INDEX + 1 >= QUESTIONS.len() {
                                bot.send_message(msg.chat.id, "the test is over".to_string()).await.unwrap();
                                
                                if let Some(ref global_vector) = GLOBAL_VECTOR {
                                    let mut vector = global_vector.lock().unwrap();
                                    vector.push(digit);
                                    println!("{:?}", *vector);

                                    let file = OpenOptions::new()
                                        .create(true)
                                        .append(true)
                                        .open("../test.csv")
                                        .expect("Failed to open file");

                                    let values_str = vector.iter().map(|value| value.to_string()).join(",");

                                    let mut csv_writer = WriterBuilder::new()
                                        .quote_style(csv::QuoteStyle::Never)
                                        .from_writer(file);
                                    csv_writer.write_record(&[values_str]).expect("Failed to write to CSV");


                                    vector.clear();
                                    
                                }
                                CURRENT_INDEX = 0;
                                dialogue.update(State::Start).await?;
                    
                            } else {
                                if let Some(ref global_vector) = GLOBAL_VECTOR {
                                    let mut vector = global_vector.lock().unwrap();
                                    vector.push(digit);
                                }
                                CURRENT_INDEX += 1;
                                bot.send_message(msg.chat.id, QUESTIONS[CURRENT_INDEX].to_string())
                                        .await
                                        .unwrap();
                            }
                        }
                    } else {
                        bot.send_message(msg.chat.id, "try digit from 1 to 5.".to_string()).await.unwrap();
                    }
                } else {                    
                    bot.send_message(msg.chat.id, "try digit from 1 to 5.".to_string()).await.unwrap();
                }     
            } else {
                bot.send_message(msg.chat.id, "try digit from 1 to 5.".to_string()).await.unwrap();
            }
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
                let message = "We kindly ask you to answer only with numbers from 1 to 5.";
                bot.send_message(msg.chat.id, message.to_string()).await?;
                unsafe { 
                    bot.send_message(msg.chat.id, QUESTIONS[CURRENT_INDEX].to_string())
                    .await
                    .unwrap();
                }

            }

            Ok(Command::Start) => {
                let message = "Hi! try to pass my test. For more information, click on /help";
                bot.send_message(msg.chat.id, message.to_string()).await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
    }
    Ok(())
}
