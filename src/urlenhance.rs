use crate::commands::CleanUrlCommand;
use crate::{app, msgfmt};
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
    if cmd.url.is_empty() {
        bot.send_message(msg.chat.id, "URL 不能为空")
            .reply_to_message_id(msg.id)
            .await?;
        return Ok(());
    }
    let cleaner = app::get().await.url_track_cleaner;
    match cleaner.do_clean(&cmd.url).await {
        Ok(res) => {
            let res = res.as_str().strip_suffix("?").unwrap_or("");
            debug!("cleaned url: {} (original={})", res, cmd.url);
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
