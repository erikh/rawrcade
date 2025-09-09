use anyhow::Result;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameList {
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub game: Vec<Game>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub folder: Vec<Folder>,
}

impl GameList {
	pub fn from_file(filename: PathBuf) -> Result<Self> {
		Ok(quick_xml::de::from_reader(std::io::BufReader::new(
			std::fs::OpenOptions::new().read(true).open(filename)?,
		))?)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
	pub name: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub desc: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub image: Option<PathBuf>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub thumbnail: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Game {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub path: Option<PathBuf>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub desc: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub image: Option<PathBuf>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub thumbnail: Option<PathBuf>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub video: Option<PathBuf>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub rating: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub releasedate: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub developer: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub publisher: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub genre: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub players: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub playcount: Option<usize>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub lastplayed: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub sortname: Option<String>,
}

#[cfg(test)]
mod tests {
	use crate::GameList;

	#[test]
	fn test_parse_xml() {
		GameList::from_file("test-gamelist.xml".into()).unwrap();
	}
}
