use bevy::prelude::*;

use crate::GameState;
pub struct MenuPlugins;

impl PluginGroup for MenuPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<MenuPlugins>()
        .add(MainMenuPlugin)
    }
}

struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu),spawn_main_menu)
        .add_systems(OnExit(GameState::MainMenu), close_menu);
    }
}

fn spawn_main_menu(
    commands: Commands
){
    println!("Spawn main menu");
}

fn close_menu(
){

}

fn name_state<T: States>(state: Res<State<T>>) {
    info!("Current state: {:?}", state.get());
}

