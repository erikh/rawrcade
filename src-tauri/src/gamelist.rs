use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameList {
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	game: Vec<ESGame>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	folder: Vec<Folder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
	name: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	desc: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	image: Option<PathBuf>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	thumbnail: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESGame {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	path: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	name: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	desc: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	image: Option<PathBuf>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	thumbnail: Option<PathBuf>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	video: Option<PathBuf>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	rating: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	releasedate: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	developer: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	publisher: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	genre: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	players: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	playcount: Option<usize>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	lastplayed: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	sortname: Option<String>,
}

#[cfg(test)]
mod tests {
	use crate::GameList;

	#[test]
	fn test_parse_xml() {
		eprintln!("{}", std::env::current_dir().unwrap().display());

		let f = std::fs::OpenOptions::new()
			.read(true)
			.open("test-gamelist.xml")
			.unwrap();

		let br = std::io::BufReader::new(f);

		let _: GameList = quick_xml::de::from_reader(br).unwrap();
	}
}
