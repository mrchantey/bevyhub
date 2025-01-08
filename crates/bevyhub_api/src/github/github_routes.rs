use crate::prelude::*;
use axum::body::Bytes;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::middleware;
use axum::response::Response;
use axum::routing::get;
use axum::Json;
use axum::Router;
use forky::server::layers;
use serde::Deserialize;

#[rustfmt::skip]
pub fn github_routes() -> AppRouter {
	Router::new()
		.route("/github/:owner/:repo/ref/:gh_ref/latest_commit",
			get(latest_commit).layer(middleware::from_fn(layers::no_cache)))
		.route("/github/:owner/:repo/ref/:gh_ref/file/*path",
			get(file))
		.route("/github/:owner/:repo/ref/:gh_ref", 
			get(crate_doc))
		.route("/github/:owner/:repo/ref/:gh_ref/scenes",
			get(all_scene_docs))
		.route("/github/:owner/:repo/ref/:gh_ref/scenes/:scene_name",
			get(scene_doc))
}

#[derive(Deserialize)]
struct GithubRouteQueryParams {
	/// The directory where the `Cargo.toml` is located
	/// The parent directory of `manifest_dir` is also used to resolve relative file locations
	/// like `scenes/my-scene.json`
	pub manifest_dir: Option<String>,
}

async fn latest_commit(
	State(_api): State<Services>,
	Path((owner, repo, branch_param)): Path<(String, String, String)>,
) -> AppResult<String> {
	let branch =
		GithubApi::resolve_branch_param(&owner, &repo, &branch_param).await?;
	let commit = GithubApi::latest_commit_hash(&owner, &repo, &branch).await?;
	Ok(commit)
}

/// Get a specific file, like `scenes/my-scene.json` from a crate
/// If a `manifest_dir` is provided, it is used to resolve relative file locations
async fn file(
	State(_api): State<Services>,
	Path((owner, repo, gh_ref_param, file_path)): Path<(
		String,
		String,
		String,
		String,
	)>,
	Query(GithubRouteQueryParams { manifest_dir }): Query<
		GithubRouteQueryParams,
	>,
) -> AppResult<Bytes> {
	let file_path = if let Some(manifest_dir) = manifest_dir {
		GithubCrateId::relative_to_manifest_dir(&manifest_dir, &file_path)
	} else {
		file_path
	};

	let gh_ref =
		GithubApi::resolve_gh_ref_param(&owner, &repo, &gh_ref_param).await?;

	let bytes = GithubApi::file(&owner, &repo, &gh_ref, &file_path).await?;

	Ok(bytes)
}

async fn crate_doc(
	State(api): State<Services>,
	Path((owner, repo, gh_ref_param)): Path<(String, String, String)>,
	Query(GithubRouteQueryParams { manifest_dir }): Query<
		GithubRouteQueryParams,
	>,
) -> AppResult<Response> {
	let commit_hash =
		GithubApi::resolve_gh_ref_param_to_hash(&owner, &repo, &gh_ref_param)
			.await?;


	let crate_id = CrateId::new_github(
		&owner,
		&repo,
		&commit_hash,
		manifest_dir.as_deref(),
	);

	let doc = api.crate_doc(&crate_id).await?;

	let res =
		append_no_cache_headers_if_latest_or_branch(Json(doc), &gh_ref_param);
	Ok(res)
}

/// Get all scenes as a [Vec<SceneDoc>] for a crate
async fn all_scene_docs(
	State(api): State<Services>,
	Path((owner, repo, gh_ref_param)): Path<(String, String, String)>,
	Query(GithubRouteQueryParams { manifest_dir }): Query<
		GithubRouteQueryParams,
	>,
) -> AppResult<Response> {
	let commit_hash =
		GithubApi::resolve_gh_ref_param_to_hash(&owner, &repo, &gh_ref_param)
			.await?;

	let crate_id = CrateId::new_github(
		&owner,
		&repo,
		&commit_hash,
		manifest_dir.as_deref(),
	);

	let doc = api.all_scene_docs(&crate_id).await?;

	let res =
		append_no_cache_headers_if_latest_or_branch(Json(doc), &gh_ref_param);
	Ok(res)
}

/// Get all scenes as a [Vec<SceneDoc>] for a crate
async fn scene_doc(
	State(api): State<Services>,
	Path((owner, repo, gh_ref_param, scene_name)): Path<(
		String,
		String,
		String,
		String,
	)>,
	Query(GithubRouteQueryParams { manifest_dir }): Query<
		GithubRouteQueryParams,
	>,
) -> AppResult<Response> {
	let commit_hash =
		GithubApi::resolve_gh_ref_param_to_hash(&owner, &repo, &gh_ref_param)
			.await?;

	let crate_id = CrateId::new_github(
		&owner,
		&repo,
		&commit_hash,
		manifest_dir.as_deref(),
	);

	let scene_id = SceneId::new(crate_id, scene_name);

	let doc = api.scene_doc(&scene_id).await?;

	let res =
		append_no_cache_headers_if_latest_or_branch(Json(doc), &gh_ref_param);
	Ok(res)
}

