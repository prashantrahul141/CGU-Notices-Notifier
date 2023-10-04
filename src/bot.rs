use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone, PartialEq, Eq, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "The following commands are supported."
)]
pub enum Command {
    #[command(description = "Help Command")]
    Help,
    #[command(description = "Subscribe to CGU Notices.")]
    Subscribe,
    #[command(description = "Unsubscibe to CGU Notices.")]
    Unsubscribe,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    match cmd {
        Command::Help => {
            bot.send_message(chat_id, Command::descriptions().to_string())
                .await?
        }
        Command::Subscribe => {
            bot.send_message(chat_id, format!("Subscibed {}", chat_id))
                .await?
        }
        Command::Unsubscribe => bot.send_message(chat_id, format!("Unsubscibed")).await?,
    };

    Ok(())
}

pub async fn run(teloxide_token: &String) {
    info!("Creating bot instance.");
    let bot = teloxide::Bot::new(teloxide_token);
    info!("Listening for messages.");
    Command::repl(bot, answer).await;
}
