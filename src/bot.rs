use teloxide::{
    dptree::deps,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
    RequestError,
};

use crate::{db, structs};

const START_MESSAGE: &str =  "Greetings👋\nWould you like to subscribe to CGU notices notifications?
            \nI will send you the notice everytime CGU posts any update on their website's notice board 😸";

const SUBSCRIBE_MESSAGE: &str = "Congratulations🎉\nYou\'re now subscribed to cgu \
                    notices notifications, I will send you the notice everytime CGU \
                    posts something on their website notice board 😼";

const UNSUBSCRIBE_MESSAGE: &str = "You\'ve been unsubscribed from CGU notices notifications, \
                    I won\'t send you notices anymore 😿\nTo subscribe again just send /subscribe.";

/// Creates a keyboard made by buttons.
fn make_keyboard() -> InlineKeyboardMarkup {
    let keyboard: Vec<Vec<InlineKeyboardButton>> = vec![vec![InlineKeyboardButton::callback(
        "Yes".to_owned(),
        "Yes".to_owned(),
    )]];
    InlineKeyboardMarkup::new(keyboard)
}

/// callback handler for keyboard button.
async fn callback_handler(
    bot: Bot,
    query_callback: CallbackQuery,
    db_metadata_collection: mongodb::Collection<structs::DbMetaData>,
) -> Result<(), RequestError> {
    if let Some(_) = query_callback.data {
        // tell telegram to remove waiting from button.
        bot.answer_callback_query(&query_callback.id).await?;
        db::add_user_to_subscribers(&query_callback.from.id.to_string(), &db_metadata_collection)
            .await;
        let _ = bot
            .send_message(query_callback.from.id, SUBSCRIBE_MESSAGE)
            .await;
    }

    Ok(())
}

/// All the Commands available to bot.
#[derive(BotCommands, Clone, PartialEq, Eq, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "You can use the following commands to interact with me 😸"
)]
pub enum Command {
    #[command(description = "Start Command")]
    Start,
    #[command(description = "Get this help message.")]
    Help,
    #[command(
        description = "Subscribe to CGU Notices.\nAnd recieve notification everytime CGU posts any notice on their website\'s notice board."
    )]
    Subscribe,
    #[command(description = "Unsubscibe to CGU Notices.")]
    Unsubscribe,
}

/// Reply callback handler.
pub async fn reply(
    bot: Bot,
    msg: Message,
    cmd: Command,
    db_metadata_collection: mongodb::Collection<structs::DbMetaData>,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    match cmd {
        Command::Start => {
            let keyboard = make_keyboard();
            let _ = bot
                .send_message(chat_id, START_MESSAGE)
                .reply_markup(keyboard)
                .await;
        }

        Command::Help => {
            bot.send_message(chat_id, Command::descriptions().to_string())
                .await?;
        }

        Command::Subscribe => {
            db::add_user_to_subscribers(&chat_id.to_string(), &db_metadata_collection).await;
            bot.send_message(chat_id, SUBSCRIBE_MESSAGE).await?;
        }

        Command::Unsubscribe => {
            db::remove_user_from_subscribers(&chat_id.to_string(), &db_metadata_collection).await;
            bot.send_message(chat_id, UNSUBSCRIBE_MESSAGE).await?;
        }
    };

    Ok(())
}

/// runs bot.
pub async fn run(teloxide_token: &String, connection_uri: &String, database_name: &String) {
    info!("Creating bot instance.");
    let bot = teloxide::Bot::new(teloxide_token);
    let db_client = db::get_client(&connection_uri).await;
    let metadata_collection = db::get_metadata_collection(&db_client, &database_name);

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(reply),
        )
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    info!("Listening for messages.");
    Dispatcher::builder(bot, handler)
        .dependencies(deps![metadata_collection])
        .default_handler(|_| async {})
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
