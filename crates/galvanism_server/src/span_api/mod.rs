mod structs;
pub use structs::*;

use colored::Colorize;
use serde::de::DeserializeOwned;

pub type Result<T> = reqwest::Result<T>;

async fn get<T>(hostname: &str, api_path: &str) -> Result<T>
where
	T: DeserializeOwned,
{
	match reqwest::get(format!("http://{hostname}/api/v1/{api_path}"))
		.await?
		.json()
		.await?
	{
		Request::Data(data) => Ok(data),
		Request::Detail(detail) => {
			let button_presses_left: String =
				match reqwest::get(format!("http://{hostname}/api/v1/status")).await {
					Ok(data) => match data.json::<Status>().await {
						Ok(data) => data.system.remaining_auth_unlock_button_presses.to_string(),
						Err(err) => err.to_string(),
					},
					Err(err) => err.to_string(),
				};
			panic!("{}",
				format!(
					"\nSpan panel {hostname}\n\
					API error: {detail}\n\
					Please press the door button on your span panel 3 times to allow API access.\n\
					You will only have to do this once, the server will register a token with the span panel once it has API access for future use.\n\
					Door button presses left: {button_presses_left}\n").red()
			);
		}
	}
}

pub async fn get_panel(hostname: &str) -> Result<Panel> {
	get(hostname, "panel").await
}

pub async fn get_circuits(hostname: &str) -> Result<Circuits> {
	get(hostname, "circuits").await
}
