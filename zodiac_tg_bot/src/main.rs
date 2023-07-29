use teloxide::{types::Message, Bot};

use teloxide::types::Me;
use teloxide::utils::command::BotCommands;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

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
    let token = dotenvy::var("TELOXIDE_TOKEN").unwrap();


    let bot = Bot::from_env();

    let notifys: Store = Arc::new(Mutex::new(HashMap::new()));


    println!("Starting...");

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(command_handler))
        .branch(dptree::case![State::Test].endpoint(test_handler))
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


    for &question in QUESTIONS {
        let cloned_bot = bot.clone();

        teloxide::repl(cloned_bot, move |message: Message, bot: AutoSend<Bot>| async move {
            bot.send_message(msg.chat.id, question.to_string()).await;
            // There are non-text messages, so we need to use pattern matching
            if let Some(text) = message.text() {
                // Echo text back into the chat
                bot.send_message(message.chat.id, "hi").await?;
            }
        
            // respond is an alias to `Ok()` with a error type compatible with teloxide
            respond(())
        }).await;
        
    }

    dialogue.update(State::Test).await?;

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
                
                bot.send_message(msg.chat.id, "test".to_string()).await?;

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
