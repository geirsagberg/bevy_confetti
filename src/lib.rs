use bevy::prelude::*;
use rand::random;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(Mouse::default())
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (calculate_mouse_position))
            .add_systems(FixedUpdate, (handle_click));
    }
}

#[derive(Resource, Debug, Default)]
struct Mouse {
    position: Vec2,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

fn handle_click(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse: Res<Mouse>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        let random_color = Color::rgb(random(), random(), random());
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                mouse.position.x,
                mouse.position.y,
                0.0,
            )),
            sprite: Sprite {
                color: random_color,
                custom_size: Some(Vec2::new(2.0, 2.0)),
                ..default()
            },
            ..default()
        });
    }
}
