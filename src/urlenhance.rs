use crate::commands::CleanUrlCommand;
use crate::{app, msgfmt};
use linkify::LinkFinder;
use log::{debug, error};
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use teloxide::utils::markdown;
use teloxide::Bot;
use url_track_cleaner::{RedirectPolicy, ReserveRule, UrlTrackCleanerBuilder};

/// 生成默认的 URL 清理器
pub(crate) fn default_cleaner() -> url_track_cleaner::UrlTrackCleaner {
    UrlTrackCleanerBuilder::new()
        .follow_redirect(RedirectPolicy::All)
        .reserve_rules(vec![ReserveRule::new_with_regex(
            r#"^http(s)?://www.bilibili.com/.*"#,
            vec!["t".to_string()],
        )
        .expect("failed to create reserve rule")])
        .build()
}

/// 处理 `CleanUrl` （/clean_url）命令
pub(crate) async fn handle_clean_url_cmd(
    bot: Bot,
    msg: Message,
    cmd: CleanUrlCommand,
) -> ResponseResult<()> {
    if let Some(replied) = &msg.reply_to_message().cloned() {
        if let Some(text) = replied.text() {
            return handle_clean_urls_in_text(bot, msg, text.clone()).await;
        } else {
            bot.send_message(msg.chat.id, "无法处理此消息")
                .reply_to_message_id(msg.id)
                .await?;
            return Ok(());
        }
    }
    if cmd.url.is_empty() {
        bot.send_message(msg.chat.id, "URL 不能为空")
            .reply_to_message_id(msg.id)
            .await?;
        return Ok(());
    }
    handle_clean_single_url(bot, msg, cmd.url).await
}

async fn handle_clean_urls_in_text(bot: Bot, msg: Message, text: &str) -> ResponseResult<()> {
    let finder = LinkFinder::new();
    let links: Vec<_> = finder.links(text).collect();

    if links.is_empty() {
        bot.send_message(msg.chat.id, "未在消息中找到 URL")
            .reply_to_message_id(msg.id)
            .await?;
        return Ok(());
    }

    let cleaner = app::get().await.url_track_cleaner;
    let mut lines: Vec<String> = vec![markdown::bold("从回复文本中解析出的链接如下："), "".into()];

    for link in links {
        let url = link.as_str().to_string();
        match cleaner.do_clean(&url).await {
            Ok(res) => {
                let res = res.as_str().trim_end_matches("?");
                debug!("cleaned url: {} (original={})", res, url);
                lines.push(format!("\\- {}", msgfmt::plain_link(res)));
            }
            Err(e) => {
                let err = e.to_string();
                error!("failed to clean url (original={}): {}", url, err);
                lines.push(format!("\\- {} 解析失败", msgfmt::plain_link(&url)));
            }
        }
    }

    bot.send_message(msg.chat.id, lines.join("\n"))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(msg.id)
        .await?;

    Ok(())
}

async fn handle_clean_single_url(bot: Bot, msg: Message, url: String) -> ResponseResult<()> {
    let cleaner = app::get().await.url_track_cleaner;
    match cleaner.do_clean(&url).await {
        Ok(res) => {
            let res = res.as_str().trim_end_matches("?");
            debug!("cleaned url: {} (original={})", res, url);
            bot.send_message(
                msg.chat.id,
                format!("清理后的 URL：{}", msgfmt::plain_link(res)),
            )
        }
        Err(e) => {
            let err = e.to_string();
            error!("failed to clean url: {}", err);
            bot.send_message(
                msg.chat.id,
                format!("清理 URL 时出现错误：{}", markdown::code_block(&err)),
            )
        }
    }
    .parse_mode(ParseMode::MarkdownV2)
    .reply_to_message_id(msg.id)
    .await?;
    Ok(())
}
