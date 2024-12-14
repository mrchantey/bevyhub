use crate::prelude::*;
use anyhow::Result;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub fn github_routes() -> AppRouter { Router::new() }

const GITHUB_BRANCH: &str = "main";

pub async fn fetch_github_raw(
	username: &str,
	repo: &str,
	file_path: &str,
) -> Result<String> {
	let url = format!(
		"https://raw.githubusercontent.com/{}/{}/{GITHUB_BRANCH}/{}",
		username, repo, file_path
	);

	let response = REQWEST_CLIENT.get(&url).send().await?;

	if !response.status().is_success() {
		anyhow::bail!("Failed to fetch file: {}", response.status());
	}

	let txt = response.text().await?;
	Ok(txt)
}

pub async fn get_latest_commit_hash(owner: &str, repo: &str) -> Result<String> {
	let url = format!(
		"https://api.github.com/repos/{}/{}/commits/main",
		owner, repo
	);

	let response = REQWEST_CLIENT.get(&url).send().await?;

	let json = response.text().await?;
	let json = serde_json::from_str::<Value>(&json)?;
	let sha = json["sha"].as_str().unwrap().to_string();
	Ok(sha)
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CommitInfo {
	sha: String,
}

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::Result;
	use sweet::*;

	#[tokio::test]
	#[ignore]
	async fn works() -> Result<()> {
		let txt =
			fetch_github_raw("mrchantey", "bevyhub", "scenes/space-scene.json")
				.await?;
		println!("{}", txt);
		Ok(())
	}

	#[tokio::test]
	async fn get_latest_commit_works() -> Result<()> {
		let res = get_latest_commit_hash("mrchantey", "bevyhub").await?;
		expect(res.len()).to_be_greater_than(16)?;
		Ok(())
	}
}
