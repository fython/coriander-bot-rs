use teloxide::macros::BotCommands;
use teloxide::prelude::*;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "支持以下命令：")]
pub(crate) enum Command {
    #[command(description = "查看机器人使用说明")]
    Help,
}

pub(crate) async fn handle_help_cmd(bot: Bot, msg: Message, _: Command) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "我的主人很懒！！！！不如直接问问他怎么用")
        .await?;
    Ok(())
}
