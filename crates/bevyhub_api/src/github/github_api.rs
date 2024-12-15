use crate::prelude::*;
use anyhow::Result;
use axum::body::Bytes;
use reqwest::Response;
use serde_json::Value;

pub struct GithubApi;

const BASE_URL: &str = "https://api.github.com";

impl GithubApi {
	/// If the ref is "latest" then resolve the latest commit hash for the default branch
	/// Otherwise return as-is
	pub async fn resolve_gh_ref_param(
		owner: &str,
		repo: &str,
		gh_ref_param: &str,
	) -> Result<String> {
		if gh_ref_param == "latest" {
			let branch = Self::default_branch(owner, repo).await?;
			Self::latest_commit_hash(owner, repo, &branch).await
		} else {
			Ok(gh_ref_param.to_string())
		}
	}

	/// 1. If the ref is "latest" then resolve the latest commit hash for the default branch
	/// 2. If the ref is a branch, resolve the latest commit hash for the branch
	/// 3. If the ref is a commit hash, return as-is
	///
	/// This only uses the `len() == 40` heuristic to determine if the ref is a commit hash
	/// so if you have a branch with 40 characters sucked in
	pub async fn resolve_gh_ref_param_to_hash(
		owner: &str,
		repo: &str,
		gh_ref_param: &str,
	) -> Result<String> {
		if gh_ref_param == "latest" {
			let branch = Self::default_branch(owner, repo).await?;
			Self::latest_commit_hash(owner, repo, &branch).await
		} else if gh_ref_param.len() != 40 {
			Self::latest_commit_hash(owner, repo, gh_ref_param).await
		} else {
			Ok(gh_ref_param.to_string())
		}
	}

	/// Currently uses raw.gitubusercontent.com but we can use REST if we need more fine-grained control
	pub async fn file(
		owner: &str,
		repo: &str,
		gh_ref: &str,
		file_path: &str,
	) -> Result<Bytes> {
		let url = format!(
			"https://raw.githubusercontent.com/{owner}/{repo}/{gh_ref}/{file_path}",
		);

		let response = Self::github_request(&url).await?;

		if !response.status().is_success() {
			anyhow::bail!("Failed to fetch file: {}", response.status());
		}

		let txt = response.bytes().await?;
		Ok(txt)
	}

	pub async fn json(
		owner: &str,
		repo: &str,
		gh_ref: &str,
		file_path: &str,
	) -> Result<Value> {
		let txt = Self::file(owner, repo, gh_ref, file_path).await?;
		let json = serde_json::from_slice::<Value>(&txt)?;
		Ok(json)
	}

	pub async fn latest_commit_hash(
		owner: &str,
		repo: &str,
		branch: &str,
	) -> Result<String> {
		let url = format!("{BASE_URL}/repos/{owner}/{repo}/branches/{branch}");

		let response = Self::github_request(&url).await?;

		let json = response.text().await?;
		let json = serde_json::from_str::<Value>(&json)?;
		let sha = json["commit"]["sha"].as_str().unwrap().to_string();
		Ok(sha)
	}

	pub async fn default_branch(owner: &str, repo: &str) -> Result<String> {
		let url = format!("{BASE_URL}/repos/{owner}/{repo}",);

		let response = Self::github_request(&url).await?;

		let json = response.text().await?;
		let json = serde_json::from_str::<Value>(&json)?;
		let default_branch =
			json["default_branch"].as_str().unwrap().to_string();
		Ok(default_branch)
	}

	async fn github_request(url: &str) -> Result<Response> {
		let token = std::env::var("GITHUB_API_TOKEN")?;
		let response = REQWEST_CLIENT
			.get(url)
			.header("Authorization", format!("Bearer {}", token))
			.send()
			.await?;
		Ok(response)
	}
}

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::Result;
	use sweet::*;

	const COMMIT_HASH: &str = "61eb2f523bfbfb41778e67770f1d115988622b80";

	#[tokio::test]
	async fn file() -> Result<()> {
		let json = GithubApi::json(
			"mrchantey",
			"bevyhub",
			"main",
			"scenes/space-scene.json",
		)
		.await?;
		expect(json.get("resources")).to_be_some()?;
		Ok(())
	}
	#[tokio::test]
	async fn file_from_hash() -> Result<()> {
		let json = GithubApi::json(
			"mrchantey",
			"bevyhub",
			COMMIT_HASH,
			"scenes/space-scene.json",
		)
		.await?;
		expect(json.get("resources")).to_be_some()?;
		Ok(())
	}

	#[tokio::test]
	async fn latest_commit() -> Result<()> {
		let res = GithubApi::latest_commit_hash("mrchantey", "bevyhub", "main")
			.await?;
		expect(res.len()).to_be_greater_than(16)?;
		Ok(())
	}
	#[tokio::test]
	async fn default_branch() -> Result<()> {
		let res = GithubApi::default_branch("mrchantey", "bevyhub").await?;
		expect(res.as_str()).to_be("main")?;
		Ok(())
	}
}
