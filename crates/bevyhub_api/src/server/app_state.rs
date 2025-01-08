use crate::prelude::Services;
use anyhow::Result;
use axum::extract::FromRef;
use forky::prelude::Uptime;


#[derive(Clone)]
pub struct AppState {
	uptime: Uptime,
	api: Services,
}

impl AppState {
	pub async fn new() -> Result<Self> {
		Ok(Self {
			uptime: Uptime::new(),
			api: Services::init().await?,
		})
	}
}

impl FromRef<AppState> for Services {
	fn from_ref(app_state: &AppState) -> Services { app_state.api.clone() }
}
impl FromRef<AppState> for Uptime {
	fn from_ref(app_state: &AppState) -> Uptime { app_state.uptime.clone() }
}
