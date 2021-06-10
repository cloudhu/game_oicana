use crate::map::Tile;
use crate::AppState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(AppState::Loading, AppState::Menu)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "sounds/background.ogg")]
    pub background: Handle<AudioSource>,
    #[asset(path = "sounds/shot.ogg")]
    pub tower_shots: Handle<AudioSource>,
    #[asset(path = "sounds/enemybreach.ogg")]
    pub enemy_breach: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/blank.png")]
    pub blank: Handle<Texture>,
    #[asset(path = "textures/towerplot.png")]
    pub tower_plot: Handle<Texture>,
    #[asset(path = "textures/tower.png")]
    pub tower: Handle<Texture>,
    #[asset(path = "textures/path.png")]
    pub path: Handle<Texture>,
    #[asset(path = "textures/castle.png")]
    pub castle: Handle<Texture>,
    #[asset(path = "textures/cloud.png")]
    pub cloud: Handle<Texture>,
    #[asset(path = "textures/spawn.png")]
    pub spawn: Handle<Texture>,
}

impl TextureAssets {
    pub fn get_handle_for_tile(&self, tile: &Tile) -> Handle<Texture> {
        match tile {
            &Tile::Empty => self.blank.clone(),
            &Tile::TowerPlot => self.tower_plot.clone(),
            &Tile::Tower => self.tower.clone(),
            &Tile::Path => self.path.clone(),
            &Tile::Castle => self.castle.clone(),
            &Tile::Cloud => self.cloud.clone(),
            &Tile::Spawn => self.spawn.clone(),
        }
    }
}
