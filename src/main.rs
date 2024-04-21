mod app;
mod commands;
mod configs;
mod msgfmt;
mod rpt;
mod urlenhance;
mod userinteract;

use crate::app::BotApp;
use crate::commands::*;
use crate::configs::BotConfigs;
use crate::rpt::RepeaterNextAction;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::{MediaKind, MediaText, MessageKind};
use teloxide::Bot;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("coriander-bot-rs is starting...");

    let bot = Bot::from_env();
    let bot_cfg = BotConfigs {
        bot_maintainer: UserId(0),
    };
    app::set(BotApp {
        repeater_state: Default::default(),
        url_track_cleaner: Arc::new(urlenhance::default_cleaner()),
    })
    .await;

    let handlers = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_cmd),
        )
        .branch(
            dptree::entry()
                .filter(|msg: Message| msg.chat.is_group() || msg.chat.is_supergroup())
                .endpoint(handle_messages_in_any_groups),
        );

    Dispatcher::builder(bot, handlers)
        .dependencies(dptree::deps![bot_cfg])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_cmd(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => handle_help_cmd(bot, msg, cmd).await,
        Command::CleanUrl(cmd) => urlenhance::handle_clean_url_cmd(bot, msg, cmd).await,
    }
}

async fn handle_messages_in_any_groups(bot: Bot, msg: Message) -> ResponseResult<()> {
    log::debug!("receive group msg: text={:?}", msg.text());

    if let MessageKind::Common(common) = &msg.kind {
        if let MediaKind::Text(MediaText { text, entities: _ }) = &common.media_kind {
            if userinteract::handle_user_do_sth_to_another(&bot, &msg)
                .await
                .is_some()
            {
                return Ok(());
            } else {
                let repeater_states = app::get().await.repeater_state;
                match repeater_states.get_next_action(msg.chat.id, text.clone()) {
                    RepeaterNextAction::Repeat => {
                        log::info!("{} needs repeat", text.clone());
                        bot.forward_message(msg.chat.id, msg.chat.id, msg.id)
                            .await?;
                    }
                    _ => (),
                }
            }
        }
    }
    Ok(())
}
