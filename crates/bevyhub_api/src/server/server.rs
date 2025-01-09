use crate::prelude::*;
use anyhow::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use forky::net::layers;
use forky::prelude::CorsState;
use forky::prelude::Uptime;
use tower_http::trace::TraceLayer;
use tower_http::trace::{
	self,
};
use tracing::Level;

pub type AppRouter = Router<AppState>;

pub async fn server() -> Result<Router> {
	let state = AppState::new().await?;


	// todo lock down cors
	const ALLOW_ANY_ORIGIN: bool = true;

	let router = Router::new()
		.route("/", get(root))
		.route("/health-check", get(health_check))
		.merge(app_routes())
		.merge(scene_routes())
		.merge(crate_routes())
		.merge(github_routes())
		.with_state(state)
		.layer(
			TraceLayer::new_for_http()
				.make_span_with(
					trace::DefaultMakeSpan::new().level(Level::INFO),
				)
				.on_response(
					trace::DefaultOnResponse::new().level(Level::INFO),
				),
		)
		.layer(middleware::from_fn_with_state(
			CorsState::new(ALLOW_ANY_ORIGIN, vec!["https://bevyhub.dev"]),
			layers::cors,
		));
	// .layer(TraceLayer::new_for_http())
	Ok(router)
}


async fn root(State(uptime): State<Uptime>) -> Html<String> {
	let version = CargoManifest::bevyhub_repo_crate_version();
	Html(format!(
		"<h1>🥁Welcome to the Bevyhub API🥁</h1><p>Verison: {}</p><p>{}</p>",
		version,
		uptime.stats()
	))
}


async fn health_check() -> (StatusCode, String) {
	let health = true;
	match health {
		true => (StatusCode::OK, "Healthy".to_string()),
		false => (StatusCode::INTERNAL_SERVER_ERROR, "Not healthy".to_string()),
	}
}
