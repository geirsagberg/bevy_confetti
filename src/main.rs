use bevy::prelude::*;
use bevy_confetti::MainPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, MainPlugin)).run();
}
