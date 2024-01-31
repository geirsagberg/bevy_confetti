use bevy::prelude::*;
use bevy_confetti::{DebugInfo, MainPlugin};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<DebugInfo>()
        .register_type::<DebugInfo>()
        .add_plugins((
            ResourceInspectorPlugin::<DebugInfo>::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            MainPlugin,
        ))
        .run();
}
