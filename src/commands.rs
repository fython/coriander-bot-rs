use teloxide::prelude::*;
use teloxide::utils::command::{BotCommands, ParseError};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case", description = "支持以下命令：")]
pub(crate) enum Command {
    #[command(description = "查看机器人使用说明")]
    Help,
    #[command(description = "清理 URL", parse_with = CleanUrlCommand::parse_to_command)]
    CleanUrl(CleanUrlCommand),
}

#[derive(Clone)]
pub(crate) struct CleanUrlCommand {
    pub url: String,
}

impl CleanUrlCommand {
    fn parse_to_command(s: String) -> Result<(Self,), ParseError> {
        Ok((CleanUrlCommand {
            url: s.trim().to_string(),
        },))
    }
}

pub(crate) async fn handle_help_cmd(bot: Bot, msg: Message, _: Command) -> ResponseResult<()> {
    let descriptions = Command::descriptions();
    bot.send_message(msg.chat.id, format!("{}", descriptions))
        .await?;
    Ok(())
}
