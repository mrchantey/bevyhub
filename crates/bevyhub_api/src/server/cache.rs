use axum::response::IntoResponse;
use axum::response::Response;
use forky::prelude::append_no_cache_headers;



/// "latest" is a special version that should not be cached
pub fn append_no_cache_headers_if_latest(
	val: impl IntoResponse,
	version: &str,
) -> Response {
	if version == "latest" {
		append_no_cache_headers(val)
	} else {
		val.into_response()
	}
}
/// "latest" is a special version that should not be cached
/// Also branches are not cached because they resolve to the latest commit
pub fn append_no_cache_headers_if_latest_or_branch(
	val: impl IntoResponse,
	version: &str,
) -> Response {
	if version == "latest" {
		append_no_cache_headers(val)
	} else if version.len() != 40 {
		append_no_cache_headers(val)
	} else {
		val.into_response()
	}
}
