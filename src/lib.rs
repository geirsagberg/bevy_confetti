#![allow(unused_parens)]
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct DebugInfo {
    entity_count: usize,
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(Mouse::default())
            // .insert_resource(Time::<Fixed>::from_seconds(0.1))
            .add_systems(Startup, (setup_camera, spawn_ground))
            .add_systems(Update, (calculate_mouse_position))
            .add_systems(FixedUpdate, (spread_joy, cleanup));
    }
}

#[derive(Resource, Debug, Default)]
struct Mouse {
    position: Vec2,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Ground {
    bitmap:
}

fn spawn_ground(mut commands: Commands) {
    commands.spawn()
}

fn calculate_mouse_position(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&Window>,
    mut mouse: ResMut<Mouse>,
) {
    let (camera_transform, camera) = camera_query.single();
    let window = window_query.single();

    let position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        .unwrap_or_default();

    mouse.position = position;
}

fn spread_joy(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse: Res<Mouse>,
    mut debug_info: ResMut<DebugInfo>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        let random_color = Color::rgb(random(), random(), random());
        for _ in 0..100 {
            debug_info.entity_count += 1;
            let direction = random::<f32>() * 2.0 * std::f32::consts::PI;

            let offset_vec_3 = Vec3::new(direction.cos(), direction.sin(), 0.0);

            let transform = Transform::from_translation(
                Vec3::new(mouse.position.x, mouse.position.y, 0.0) + offset_vec_3 * 10.0,
            );

            commands
                .spawn(SpriteBundle {
                    transform,
                    sprite: Sprite {
                        color: random_color,
                        custom_size: Some(Vec2::new(2.0, 2.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                // .insert(CollisionGroups::new(Group::NONE, Group::NONE))
                .insert(Collider::cuboid(1.0, 1.0))
                .insert(Velocity::linear(
                    offset_vec_3.truncate() * 500.0 * (random::<f32>() + 0.5),
                ));
        }
    }
}

fn cleanup(
    mut commands: Commands,
    mut debug_info: ResMut<DebugInfo>,
    query: Query<(&Transform, Entity)>,
    window: Query<&Window>,
) {
    let height = window.single().resolution.height();
    for (transform, entity) in &query {
        if transform.translation.y < -height / 2.0 {
            debug_info.entity_count -= 1;
            commands.entity(entity).despawn();
        }
    }
}
