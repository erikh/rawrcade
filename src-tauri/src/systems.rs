use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::Game;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemList {
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub system: Vec<System>,
}

impl SystemList {
	pub fn from_file(filename: PathBuf) -> Result<Self> {
		Ok(quick_xml::de::from_reader(std::io::BufReader::new(
			std::fs::OpenOptions::new().read(true).open(filename)?,
		))?)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
	pub name: String,
	pub fullname: String,
	pub path: PathBuf,
	pub extension: String,
	pub command: String,
	pub platform: String,
	#[serde(skip_deserializing)]
	pub gamelist: Vec<Game>,
}
