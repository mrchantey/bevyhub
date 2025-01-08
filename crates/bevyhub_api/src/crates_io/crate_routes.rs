use crate::prelude::*;
use axum::body::Bytes;
use axum::extract::Path;
use axum::extract::State;
use axum::middleware;
use axum::response::Json;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use forky::net::layers;
use semver::Version;

#[rustfmt::skip]
pub fn crate_routes() -> AppRouter {
	Router::new()
		.route("/crates/:crate_name/versions",
			get(all_versions).layer(middleware::from_fn(layers::no_cache)))
		.route("/crates/:crate_name/versions/:version/file/*path",
			get(file))
		.route("/crates/:crate_name/versions/:version", 
			get(crate_doc))
		.route("/crates/:crate_name/versions/:version/scenes",
			get(all_scene_docs))
		.route("/crates/:crate_name/versions/:version/scenes/:scene_name",
			get(scene_doc))
}


/// Get a specific file, like `scenes/my-scene.json` from a crate
async fn file(
	State(api): State<Services>,
	Path((crate_name, version, file_path)): Path<(String, String, String)>,
) -> AppResult<Bytes> {
	let version = Version::parse(&version)?;
	let crate_id = CratesIoCrateId::new(&crate_name, version);
	let bytes =
		CratesIoFiles::get_crate_file(&api, &crate_id, &file_path).await?;

	Ok(bytes)
}

/// Get all versions of a crate
async fn all_versions(
	State(api): State<Services>,
	Path(crate_name): Path<String>,
) -> AppResult<Json<Vec<Version>>> {
	let versions = api.registry().versions(&crate_name).await?;
	Ok(Json(versions))
}

/// Get a [CrateDoc]
async fn crate_doc(
	State(api): State<Services>,
	Path((crate_name, version_param)): Path<(String, String)>,
) -> AppResult<Response> {
	let version = api
		.registry()
		.resolve_version(&crate_name, &version_param)
		.await?;

	let crate_id = CrateId::new_crates_io(&crate_name, version);

	let doc = api.crate_doc(&crate_id).await?;
	let res = append_no_cache_headers_if_latest(Json(doc), &version_param);
	Ok(res)
}

/// Get all scenes as a [Vec<SceneDoc>] for a crate
async fn all_scene_docs(
	State(api): State<Services>,
	Path((crate_name, version_param)): Path<(String, String)>,
) -> AppResult<Response> {
	let version = api
		.registry()
		.resolve_version(&crate_name, &version_param)
		.await?;

	let crate_id = CrateId::new_crates_io(&crate_name, version);

	let docs = api.all_scene_docs(&crate_id).await?;
	let res = append_no_cache_headers_if_latest(Json(docs), &version_param);
	Ok(res)
}

/// Get a [SceneDoc] for a crate
async fn scene_doc(
	State(api): State<Services>,
	Path((crate_name, version_param, scene_name)): Path<(
		String,
		String,
		String,
	)>,
) -> AppResult<Response> {
	let version = api
		.registry()
		.resolve_version(&crate_name, &version_param)
		.await?;
	let scene_id = SceneId::new_crates_io(&crate_name, version, scene_name);
	let doc = api.scene_doc(&scene_id).await?;
	let res = append_no_cache_headers_if_latest(Json(doc), &version_param);
	Ok(res)
}
