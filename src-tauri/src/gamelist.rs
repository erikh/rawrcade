use anyhow::Result;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::Game;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameList {
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub game: Vec<ESGame>,
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

impl Into<Game> for ESGame {
	fn into(self) -> Game {
		// NOTE: use the name, if not the name, use the basename of the path of the ROM, if not that,
		// title it "Unnamed rom" and wash your hands of the matter.

		Game {
			name: self.name.unwrap_or(self.path.map_or(
				"Unnamed ROM".into(),
				|x| {
					x.file_name().unwrap().to_str().unwrap().to_string()
				},
			)),
			desc: self.desc,
			rating: self.rating,
			releasedate: self.releasedate,
			developer: self.developer,
			publisher: self.publisher,
			genre: self.genre,
			players: self.players,
			playcount: self.playcount,
			lastplayed: self.lastplayed,
		}
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESGame {
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
