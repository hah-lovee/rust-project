use std::sync::Mutex;
use std::env;

use sqlx::postgres::PgPool;
use sqlx::Pool;
use sqlx::query;

use teloxide::{types::{Message, Me}, Bot};
use teloxide::utils::command::BotCommands;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use dotenv::dotenv;



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
async fn main() -> Result<(), sqlx::Error> {
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

    Ok(())
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
                    // Здесь в бдшку запрос на отправку должен быть + vec.clean()
                            if CURRENT_INDEX + 1 >= QUESTIONS.len() {
                                bot.send_message(msg.chat.id, "тест окончен".to_string()).await.unwrap();
                                
                                if let Some(ref global_vector) = GLOBAL_VECTOR {
                                    let mut vector = global_vector.lock().unwrap();
                                    vector.push(digit);
                                    println!("{:?}", *vector);

                                    insert_values().await?;

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

async fn insert_values() -> Result<(), sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = PgPool::connect(&db_url).await?;

    unsafe{
        if let Some(ref global_vector) = GLOBAL_VECTOR {
            let mut vector = global_vector.lock().unwrap();

            let first_q = vector[0];
            let second_q = vector[1];
            let third_q = vector[2];
            let fourth_q = vector[3];
            let fifth_q = vector[4];
            let sixth_q = vector[5];
            let seventh_q = vector[6];
            let eighth_q = vector[7];
            let ninth_q = vector[8];
            let tenth_q = vector[9];

            let query = "INSERT INTO test (q1, q2, q3, q4, q5, q6, q7, q8, q9, q10)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)";
            sqlx::query(query)
                .bind(first_q)
                .bind(second_q)
                .bind(third_q)
                .bind(fourth_q)
                .bind(fifth_q)
                .bind(sixth_q)
                .bind(seventh_q)
                .bind(eighth_q)
                .bind(ninth_q)
                .bind(tenth_q)
                .execute(&pool)
                .await?;
            
        }   
    }
    Ok(())
}
