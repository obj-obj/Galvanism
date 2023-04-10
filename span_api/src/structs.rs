use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Branch {
	pub id: u8,
	pub instant_power_w: f32,
	pub imported_active_energy_wh: f32,
	pub exported_active_energy_wh: f32,
}

/// `GET` `/api/v1/panel`
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Panel {
	pub instant_grid_power_w: f32,
	pub feedthrough_power_w: f32,
	pub branches: Vec<Branch>,
}
