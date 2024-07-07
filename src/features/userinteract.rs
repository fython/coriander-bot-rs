use crate::commands::ReplaceCommand;
use crate::msgfmt::markup_username_with_link;
use std::any::Any;
use std::ptr::replace;
use teloxide::prelude::*;
use teloxide::types::{MediaKind, MediaText, MessageKind, ParseMode};

/// 处理用户对另一个用户的模拟指令行为
///
/// 当用户发送类似 `/do_sth` 的消息时，机器人会回复类似 `@user1 do_sth 了 @user2` 的消息
/// 若消息不符合条件时，此函数会返回 None。此外，发送失败时不会传递错误到上游
pub(crate) async fn handle_user_do_sth_to_another(bot: &Bot, msg: &Message) -> Option<()> {
    let from_user = msg.from();
    let reply_user = msg.reply_to_message().and_then(|reply| reply.from());
    let text = msg.text();
    if text.is_none() || from_user.is_none() || reply_user.is_none() {
        return None;
    }
    let text = text.unwrap();
    if text.starts_with("/") && text.find("@").is_none() {
        let act = text.strip_prefix("/");
        if act.is_none() {
            return None;
        }
        let acts: Vec<&str> = act.unwrap().split_ascii_whitespace().into_iter().collect();
        if acts.len() == 0 || acts[0] == "me" {
            return None;
        }
        let mut text = format!(
            "{} {}了 {}",
            markup_username_with_link(from_user.unwrap()),
            acts[0],
            markup_username_with_link(reply_user.unwrap())
        );
        if acts.len() > 1 {
            text.push_str(&format!(" {}", acts[1]));
        }
        let res = bot
            .send_message(msg.chat.id, text)
            .parse_mode(ParseMode::MarkdownV2)
            .await;
        if res.is_err() {
            log::error!("failed to send message: {:?}", res.err());
        }
        // TODO 统计行为次数
        return Some(());
    }
    return None;
}

/// 处理用户替换消息中的关键词
///
/// 当用户回复一条消息并发送类似 `/replace keyword replacement` 的消息时，
/// 机器人会将被回复的消息中的 `keyword` 替换为 `replacement` 并回复。
/// 被回复的消息必须是纯文本消息，否则跳过处理。
pub(crate) async fn handle_user_replace_words(
    bot: &Bot,
    msg: &Message,
    cmd: ReplaceCommand,
) -> ResponseResult<()> {
    let reply_msg = msg.reply_to_message();
    if reply_msg.is_none() {
        bot.send_message(msg.chat.id, "此命令需要指定一条消息回复")
            .reply_to_message_id(msg.id)
            .await?;
        return Ok(());
    }
    // 限制仅支持纯文本消息
    if let MessageKind::Common(common) = &reply_msg.unwrap().kind {
        if let MediaKind::Text(MediaText { text, entities: _ }) = &common.media_kind {
            if msg.from().is_none() {
                // ignored
                return Ok(());
            }
            let from = msg.from().unwrap();
            let replaced = text.replace(&cmd.keyword, &cmd.replacement);
            let replacer = markup_username_with_link(from);
            bot.send_message(msg.chat.id, format!("{} ：{}", replacer, replaced))
                .parse_mode(ParseMode::MarkdownV2)
                .reply_to_message_id(msg.id)
                .await?;
            return Ok(());
        }
    }
    bot.send_message(msg.chat.id, "无法处理此消息")
        .reply_to_message_id(msg.id)
        .await?;
    Ok(())
}
