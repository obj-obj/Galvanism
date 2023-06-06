use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};

// Common data types
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Request<T> {
	Detail(Detail),
	Data(T),
}

#[derive(Deserialize, Debug)]
pub struct Detail {
	pub detail: String,
}
impl Display for Detail {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.detail)
	}
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
	Open,
	Closed,
}

/// `GET` `/api/v1/status`
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
	pub software: Software,
	pub system: System,
	pub network: Network,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Software {
	pub firmware_version: String,
	pub update_status: String,
	pub env: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct System {
	pub manufacturer: String,
	pub serial: String,
	pub model: String,
	pub door_state: State,
	pub remaining_auth_unlock_button_presses: u8,
	pub uptime: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Network {
	pub eth_0_link: bool,
	pub wlan_link: bool,
	pub wwan_link: bool,
}

/// `GET` `/api/v1/panel`
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Panel {
	pub main_relay_state: State,
	pub main_meter_energy: MainMeterEnergy,
	pub instant_grid_power_w: f64,
	pub feedthrough_power_w: f64,
	pub feedthrough_energy: FeedthroughEnergy,
	pub grid_sample_start_ms: i32,
	pub grid_sample_end_ms: i32,
	pub dsm_grid_state: DsmGridState,
	pub dsn_state: DsmState,
	pub current_run_config: CurrentRunConfig,
	pub branches: Vec<Branch>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainMeterEnergy {
	pub produced_energy_wh: f64,
	pub consumed_energy_wh: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeedthroughEnergy {
	pub produced_energy_wh: f64,
	pub consumed_energy_wh: f64,
}

// These 3 following enums are unfinished - not all variants are known yet
#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DsmGridState {
	DsmGridUp,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DsmState {
	DsmOnGrid,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CurrentRunConfig {
	PanelOnGrid,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Branch {
	pub id: u8,
	pub relay_state: State,
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
	pub relay_state: State,
	pub instant_power_w: f64,
	pub instant_power_update_time_s: i64,
	pub produced_energy_wh: f64,
	pub consumed_energy_wh: f64,
	pub energy_accum_update_time_s: i64,
	pub tabs: Vec<u8>,
	pub priority: Priority,
	pub is_user_controllable: bool,
	pub is_sheddable: bool,
	pub is_never_backup: bool,
}

// Unfinished (not all variants are known yet)
#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Priority {
	MustHave,
}
