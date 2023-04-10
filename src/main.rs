use mdns_sd::{ServiceDaemon, ServiceEvent};
use span_api::get_panel;
use std::{
	thread,
	time::{Duration, Instant},
};
use tokio::task::JoinHandle;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let receiver = ServiceDaemon::new()?.browse("_span._tcp.local.")?;
	let mut hostnames: Vec<String> = Vec::new();
	let time = Instant::now();
	while time.elapsed().as_secs() < 6 {
		match receiver.recv_async().await? {
			ServiceEvent::ServiceResolved(info) => {
				let hostname = info.get_hostname();
				// Every hostname has an extra period at the end that must be removed
				hostnames.push(hostname[..hostname.len() - 1].into());
			}
			_ => {}
		}
	}

	loop {
		let mut handles: Vec<JoinHandle<Result<f32, reqwest::Error>>> = Vec::new();
		for hostname in hostnames.clone() {
			handles.push(tokio::spawn(async move {
				let power = get_panel(&hostname).await?.instant_grid_power_w;
				println!("{hostname}: {power}W");
				Ok(power)
			}));
		}

		let mut power = 0.0;
		for handle in handles {
			power += handle.await??;
		}
		println!("Total power draw: {power}W");
		thread::sleep(Duration::from_secs(1));
	}
}
