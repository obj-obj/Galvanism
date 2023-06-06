mod span_api;

use lazy_static::lazy_static;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use reqwest::Client;
use span_api::{get_circuits, get_panel};
use std::{
	cmp::Ordering,
	io, thread,
	time::{Duration, Instant},
};
use tokio::sync::mpsc;
use tui::{backend::CrosstermBackend, Terminal};

type Error = Box<dyn std::error::Error + Send + Sync>;

lazy_static! {
	pub static ref CLIENT: Client = Client::builder()
		.gzip(true)
		.brotli(true)
		.deflate(true)
		.use_rustls_tls()
		.build()
		.unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	let (tx, mut rx) = mpsc::channel::<String>(10);

	// Zeroconf discovery thread
	tokio::spawn(async move {
		let receiver = ServiceDaemon::new()
			.unwrap()
			.browse("_span._tcp.local.")
			.unwrap();
		loop {
			if let ServiceEvent::ServiceResolved(info) = receiver.recv_async().await.unwrap() {
				let hostname = info.get_hostname();
				// Every hostname has an extra period at the end that must be removed
				tx.send(hostname[..hostname.len() - 1].to_string())
					.await
					.unwrap();
			}
		}
	});

	let stdout = io::stdout();
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	let mut domains = Vec::new();
	loop {
		let time = Instant::now();

		while let Ok(domain) = rx.try_recv() {
			if !domains.contains(&domain) {
				domains.push(domain);
			}
		}

		let mut panel_handles = Vec::new();
		let mut circuit_handles = Vec::new();
		for domain in &domains {
			let panel_domain = domain.clone();
			let circuit_domain = domain.clone();
			panel_handles.push(tokio::spawn(async move { get_panel(&panel_domain).await }));
			circuit_handles.push(tokio::spawn(
				async move { get_circuits(&circuit_domain).await },
			));
		}
		let mut panels = Vec::new();
		for handle in panel_handles {
			panels.push(handle.await??);
		}
		let mut circuits = Vec::new();
		for handle in circuit_handles {
			for circuit in handle.await??.circuits.into_values() {
				circuits.push(circuit);
			}
		}
		circuits.sort_unstable_by(|a, b| {
			a.instant_power_w
				.partial_cmp(&b.instant_power_w)
				.unwrap_or(Ordering::Equal)
		});

		let mut grid_power = 0.0;
		for panel in &panels {
			grid_power += panel.instant_grid_power_w;
		}

		terminal.clear()?;
		println!("Total: {grid_power}W");
		for circuit in circuits.iter().take((terminal.size()?.height - 2).into()) {
			println!("{}: {}W", circuit.name, circuit.instant_power_w);
		}

		thread::sleep(Duration::from_secs(1) - time.elapsed());
	}
}
