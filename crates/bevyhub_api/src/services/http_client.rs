use once_cell::sync::Lazy;
use reqwest::Client;


/// Crates.io asks for contact information in the User-Agent header
/// for api usage. 
static USER_AGENT: &str = "contact:github.com/mrchantey/bevyhub";

pub static REQWEST_CLIENT: Lazy<Client> = Lazy::new(|| {
	Client::builder()
		.user_agent(USER_AGENT)
		.build()
		.expect("Failed to build reqwest client")
});
