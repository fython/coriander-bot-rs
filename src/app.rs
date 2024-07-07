use crate::features::rpt::RepeaterStates;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;
use url_track_cleaner::UrlTrackCleaner;

#[derive(Clone, Debug)]
pub(crate) struct BotApp {
    pub repeater_state: Arc<RepeaterStates>,
    pub url_track_cleaner: Arc<UrlTrackCleaner>,
}

impl BotApp {
    pub fn new(states: RepeaterStates, cleaner: UrlTrackCleaner) -> Self {
        BotApp {
            repeater_state: Arc::new(states),
            url_track_cleaner: Arc::new(cleaner),
        }
    }
}

impl Default for BotApp {
    fn default() -> Self {
        BotApp {
            repeater_state: Arc::new(RepeaterStates::default()),
            url_track_cleaner: Arc::new(UrlTrackCleaner::default()),
        }
    }
}

static APP: Lazy<Mutex<BotApp>> = Lazy::new(|| Mutex::new(BotApp::default()));

/// Set the global bot app instance
///
/// This function can only be called once when program starts
pub(crate) async fn set(app: BotApp) {
    APP.lock().await.clone_from(&app);
}

/// Get the global bot app instance
pub(crate) async fn get() -> BotApp {
    APP.lock().await.clone()
}
