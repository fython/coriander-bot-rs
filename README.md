coriander-bot-rs
======

烧饼 Telegram Bot 的重构版本

# 运行

1. 将项目克隆至本地
2. 设置环境变量 `TELOXIDE_TOKEN` 为 Telegram Bot Token
3. (可选) 设置环境变量 `TELOXIDE_PROXY` 为本地代理
4. 执行以下命令
   ```shell
   cargo run --package coriander-bot-rs --bin coriander-bot-rs
   ```

# Feature list

- [x] 基本运行
- [x] 复读机
    - [ ] 复读表情包
    - [ ] 按群内全局记录时间判断是否复读（当前仅判断相同文本消息是否在指定间隔时间内出现，对于冷群略长时间的复读体验不好）
- [x] 清理链接追踪参数（`/clean_url`)
    - [x] 支持回复文本消息批量清理追踪参数
    - [ ] 支持在群组中自动清理链接追踪参数
- [ ] 其他特性

# Licenses

MIT License
