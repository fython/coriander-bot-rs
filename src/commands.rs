use shlex::Shlex;
use std::io;
use teloxide::prelude::*;
use teloxide::utils::command::{BotCommands, ParseError};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case", description = "支持以下命令：")]
pub(crate) enum Command {
    #[command(description = "查看机器人使用说明")]
    Help,
    #[command(description = "清理 URL", parse_with = CleanUrlCommand::parse_to_command)]
    CleanUrl(CleanUrlCommand),
    #[command(description = "替换原文中的关键字并发送", parse_with = ReplaceCommand::parse_to_command)]
    Replace(ReplaceCommand),
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

#[derive(Clone)]
pub(crate) struct ReplaceCommand {
    pub keyword: String,
    pub replacement: String,
}

impl ReplaceCommand {
    fn parse_to_command(s: String) -> Result<(Self,), ParseError> {
        let mut l = Shlex::new(&s);
        let parts: Vec<String> = l.by_ref().collect();
        if l.had_error {
            return Err(ParseError::IncorrectFormat(
                io::Error::other("parse arguments error").into(),
            ));
        }
        #[allow(non_upper_case_globals)]
        const expected: usize = 2;
        let found = parts.len();
        if found != expected {
            let message = "replace args count should be 2".to_string();
            return Err(if found > expected {
                ParseError::TooFewArguments {
                    expected,
                    found,
                    message,
                }
            } else {
                ParseError::TooFewArguments {
                    expected,
                    found,
                    message,
                }
            });
        }
        Ok((ReplaceCommand {
            keyword: parts[0].clone(),
            replacement: parts[1].clone(),
        },))
    }
}

pub(crate) async fn handle_help_cmd(bot: Bot, msg: Message, _: Command) -> ResponseResult<()> {
    let descriptions = Command::descriptions();
    bot.send_message(msg.chat.id, format!("{}", descriptions))
        .await?;
    Ok(())
}
