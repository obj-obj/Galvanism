mod structs;
pub use structs::*;

pub async fn get_panel(hostname: &String) -> Result<Panel, reqwest::Error> {
	reqwest::get(format!("http://{hostname}/api/v1/panel"))
		.await?
		.json()
		.await
}

pub async fn get_circuits(hostname: &String) -> Result<Circuits, reqwest::Error> {
	reqwest::get(format!("http://{hostname}/api/v1/circuits"))
		.await?
		.json()
		.await
}
