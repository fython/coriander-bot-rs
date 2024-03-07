use crate::msgfmt::markup_username_with_link;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

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
