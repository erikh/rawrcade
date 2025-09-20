use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const DEFAULT_CONFIG_FILENAME: &str = "rawrcade/config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
	Trace,
	Debug,
	Info,
	Warn,
	Error,
}

impl Into<tracing::Level> for LogLevel {
	fn into(self) -> tracing::Level {
		match self {
			Self::Trace => tracing::Level::TRACE,
			Self::Debug => tracing::Level::DEBUG,
			Self::Info => tracing::Level::INFO,
			Self::Warn => tracing::Level::WARN,
			Self::Error => tracing::Level::ERROR,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub swap_confirm: bool,
	pub start_fullscreen: bool,
	pub theme: Option<String>,
	pub enable_keyboard: bool,
	pub log_level: LogLevel,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			swap_confirm: false,
			start_fullscreen: true,
			theme: None,
			enable_keyboard: false,
			log_level: LogLevel::Debug,
		}
	}
}

impl Config {
	pub fn from_file(filename: &PathBuf) -> Result<Self> {
		let f = std::fs::OpenOptions::new().read(true).open(filename)?;

		Ok(serde_json::from_reader(f)?)
	}

	pub fn to_file(&self, filename: &PathBuf) -> Result<()> {
		if let Some(parent) = filename.parent() {
			std::fs::create_dir_all(parent)?;
		}

		let f = std::fs::OpenOptions::new()
			.create(true)
			.write(true)
			.open(filename)?;
		Ok(serde_json::to_writer_pretty(&f, self)?)
	}
}
