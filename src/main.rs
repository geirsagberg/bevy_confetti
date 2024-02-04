use bevy::prelude::*;
use bevy_confetti::{DebugInfo, MainPlugin};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<DebugInfo>()
        .register_type::<DebugInfo>()
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: 1.0 / 60.0,
                substeps: 1,
            },
            ..default()
        })
        .add_plugins((
            ResourceInspectorPlugin::<DebugInfo>::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0).in_fixed_schedule(),
            MainPlugin,
        ))
        .run();
}
