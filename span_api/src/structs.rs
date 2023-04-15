use serde::Deserialize;
use std::collections::HashMap;

/// `GET` `/api/v1/panel`
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Panel {
	pub instant_grid_power_w: f64,
	pub feedthrough_power_w: f64,
	pub branches: Vec<Branch>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Branch {
	pub id: u8,
	pub instant_power_w: f64,
	pub imported_active_energy_wh: f64,
	pub exported_active_energy_wh: f64,
}

/// `GET` `/api/v1/circuits`
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Circuits {
	pub circuits: HashMap<String, Circuit>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Circuit {
	pub id: String,
	pub name: String,
	pub instant_power_w: f64,
	pub instant_power_update_time_s: i64,
	pub produced_energy_wh: f64,
	pub consumed_energy_wh: f64,
	pub energy_accum_update_time_s: i64,
	pub tabs: Vec<u8>,
	pub is_user_controllable: bool,
	pub is_sheddable: bool,
	pub is_never_backup: bool,
}
