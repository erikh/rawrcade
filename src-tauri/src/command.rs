use crate::SettingTypes;

use super::{App, Orientation, System};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn setting_value(
	state: State<'_, App>, setting: usize,
) -> std::result::Result<String, ()> {
	tracing::debug!("settings value requested for setting: {}", setting);

	let res = match state.setting_value(setting).await {
		SettingTypes::Boolean(b) => serde_json::to_string(&b),
		SettingTypes::OptionString(os) => serde_json::to_string(&os),
	};

	Ok(res.unwrap())
}

#[tauri::command]
pub fn setting_types(state: State<'_, App>) -> Vec<String> {
	tracing::debug!("settings types requested");
	state.settings_types()
}

#[tauri::command]
pub fn settings_menu(state: State<'_, App>) -> Vec<String> {
	tracing::debug!("settings menu requested");
	state
		.settings_menu()
		.iter()
		.map(ToString::to_string)
		.collect()
}

#[tauri::command]
pub fn menu(state: State<'_, App>) -> Vec<String> {
	state.menu()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
	#[default]
	Image,
	Thumbnail,
	Video,
}

#[tauri::command]
pub async fn current_asset(
	state: State<'_, App>, asset_type: AssetType,
) -> std::result::Result<Option<PathBuf>, ()> {
	let systems = state.all_systems.lock().await.system.clone();
	let orientation = state.orientation.lock().await;
	let current_system = &systems[orientation.system_index];
	let current_game = &current_system.gamelist[orientation.gamelist_index];
	let filename: Option<PathBuf> = match asset_type {
		AssetType::Image => current_game.image.clone(),
		AssetType::Thumbnail => current_game.thumbnail.clone(),
		AssetType::Video => current_game.video.clone(),
	};
	Ok(filename)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextType {
	#[default]
	Description,
	Rating,
	ReleaseDate,
	Developer,
	Publisher,
	Genre,
	Players,
	PlayCount,
	LastPlayed,
}

#[tauri::command]
pub async fn current_text(
	state: State<'_, App>, text_type: TextType,
) -> std::result::Result<Option<String>, ()> {
	let systems = state.all_systems.lock().await.system.clone();
	let orientation = state.orientation.lock().await;
	let current_system = &systems[orientation.system_index];
	let current_game = current_system.gamelist[orientation.gamelist_index].clone();
	Ok(match text_type {
		TextType::Description => current_game.desc,
		TextType::Rating => current_game.rating,
		TextType::ReleaseDate => current_game.releasedate,
		TextType::Developer => current_game.developer,
		TextType::Publisher => current_game.publisher,
		TextType::Genre => current_game.genre,
		TextType::Players => current_game.players,
		TextType::PlayCount => current_game.playcount.map(|x| x.to_string()),
		TextType::LastPlayed => current_game.lastplayed,
	})
}

#[tauri::command]
pub async fn all_systems(state: State<'_, App>) -> std::result::Result<Vec<System>, ()> {
	Ok(state.all_systems.clone().lock_owned().await.system.clone())
}

#[tauri::command]
pub async fn current_orientation(state: State<'_, App>) -> std::result::Result<Orientation, ()> {
	Ok(state.orientation.clone().lock_owned().await.clone())
}
