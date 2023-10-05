use teloxide::{dptree::deps, prelude::*, utils::command::BotCommands};

use crate::{db, structs};

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

pub async fn reply(
    bot: Bot,
    msg: Message,
    cmd: Command,
    db_metadata_collection: mongodb::Collection<structs::DbMetaData>,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    match cmd {
        Command::Help => {
            bot.send_message(chat_id, Command::descriptions().to_string())
                .await?;
        }
        Command::Subscribe => {
            db::add_user_to_subscribers(&chat_id.to_string(), &db_metadata_collection).await;
            bot.send_message(
                chat_id,
                format!(
                    "Congratulations!\nYou\'re now subscribed to cgu \
                    notices notifications, I will send you the notice everytime CGU \
                    posts something on their website notice board."
                ),
            )
            .await?;
        }
        Command::Unsubscribe => {
            db::remove_user_from_subscribers(&chat_id.to_string(), &db_metadata_collection).await;
            bot.send_message(
                chat_id,
                format!(
                    "You\'ve been unsubscribed from CGU notices notificaitons, \
                    I won\'t send you notices anymore.\nTo subscribe again just send /subscribe"
                ),
            )
            .await?;
        }
    };

    Ok(())
}

pub async fn run(teloxide_token: &String, connection_uri: &String, database_name: &String) {
    info!("Creating bot instance.");
    let bot = teloxide::Bot::new(teloxide_token);
    let db_client = db::get_client(&connection_uri).await;
    let metadata_collection = db::get_metadata_collection(&db_client, &database_name);

    let handler = dptree::entry().branch(
        Update::filter_message()
            .filter_command::<Command>()
            .endpoint(reply),
    );

    info!("Listening for messages.");
    Dispatcher::builder(bot, handler)
        .dependencies(deps![metadata_collection])
        .default_handler(|_| async {})
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
