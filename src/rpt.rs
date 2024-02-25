use std::time::SystemTime;
use chrono::{DateTime, TimeDelta, Utc};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use teloxide::prelude::ChatId;

/// 复读机状态
///
/// 各群组单独计算群内相同语句的出现间隔和上次复读时间
///
/// 特性描述：
/// 当群组内有小于 N 长度的文本消息在 M 时间间隔内重复出现第二次，则机器人复读一次，
/// 并记录下复读发生的时间，在相同条件下距离上次复读发生的 R 时间间隔内不再复读，
/// 直至超时。
pub(crate) struct RepeaterStates {
    /// 复读文本的最大长度，超过此长度则不进行统计和复读
    pub max_text_length: u32,
    /// 等待用户复读最大间隔，若第二条复读超过此间隔不会进行复读
    pub max_wait_repeat_duration: TimeDelta,
    /// 最近复读间隔，若上次复读时间距离当前小于此间隔则不会进行复读
    pub min_recent_repeat_duration: TimeDelta,
    /// 各群组状态记录，定义详见 RepeaterGroupState
    pub groups: DashMap<ChatId, RepeaterGroupState>,
}

pub(crate) static REPEATER_STATES: Lazy<RepeaterStates> = Lazy::new(|| {
    RepeaterStates {
        max_text_length: 3 * 10,
        max_wait_repeat_duration: TimeDelta::seconds(15),
        min_recent_repeat_duration: TimeDelta::seconds(30),
        groups: DashMap::new(),
    }
});

impl RepeaterStates {
    pub fn get_next_action(&self, chat_id: ChatId, text: String) -> RepeaterNextAction {
        log::debug!("text.len(): {}", text.len());
        if text.len() > self.max_text_length as usize {
            return RepeaterNextAction::Nothing;
        }
        self.groups.entry(chat_id)
            .or_default()
            .get_next_action(text, self.max_wait_repeat_duration, self.min_recent_repeat_duration)
    }
}

/// 单个群组内复读状态
#[derive(Debug)]
pub(crate) struct RepeaterGroupState {
    pub text_states: DashMap<String, RepeatedTextState>,
}

impl Default for RepeaterGroupState {
    fn default() -> Self {
        RepeaterGroupState {
            text_states: DashMap::new(),
        }
    }
}

impl RepeaterGroupState {
    pub fn get_next_action(&self, text: String,
                           max_wait_repeat_duration: TimeDelta,
                           min_recent_repeat_duration: TimeDelta) -> RepeaterNextAction {
        let now = DateTime::from(SystemTime::now());
        let mut state = self.text_states.entry(text).or_default();
        let mut res = RepeaterNextAction::Nothing;
        if let Some(last_msg_time) = state.last_msg_time {
            // 若当前时间距离上次接收相同文本的时间小于最大等待复读间隔，则可以继续判断是否复读
            if now - last_msg_time <= max_wait_repeat_duration {
                // 将状态设置为可复读，再继续判断
                res = RepeaterNextAction::Repeat;
                // 若上次发送过复读文本且当前时间距离上次复读时间小于最小间隔，则取消复读
                if let Some(last_send_time) = state.last_send_time {
                    if now - last_send_time <= min_recent_repeat_duration {
                        res = RepeaterNextAction::Nothing;
                    }
                }
            }
        }
        state.last_msg_time = Some(now);
        // 若状态设置为可复读，则更新此文本在群组内的复读时间
        if let RepeaterNextAction::Repeat = res {
            state.last_send_time = Some(now);
        }
        res
    }
}

pub(crate) enum RepeaterNextAction {
    Nothing,
    Repeat,
}

/// 复读文本状态
#[derive(Debug, Clone)]
pub(crate) struct RepeatedTextState {
    pub last_msg_time: Option<DateTime<Utc>>,
    pub last_send_time: Option<DateTime<Utc>>,
}

impl Default for RepeatedTextState {
    fn default() -> Self {
        RepeatedTextState {
            last_msg_time: None,
            last_send_time: None,
        }
    }
}
