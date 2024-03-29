use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::BevyDefault,
    },
};
use bevy_rapier2d::prelude::*;
use rand::random;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct DebugInfo {
    entity_count: usize,
    mouse_position: Vec2,
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(Mouse::default())
            // .insert_resource(Time::<Fixed>::from_seconds(0.1))
            .add_systems(Startup, (setup_camera, spawn_ground))
            .add_systems(Update, (calculate_mouse_position, cleanup))
            .add_systems(
                FixedUpdate,
                (spread_joy, paint_ground).after(PhysicsSet::Writeback),
            );
    }
}

#[derive(Resource, Debug, Default)]
struct Mouse {
    position: Vec2,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct QuantizedColor {
    r: u8,
    g: u8,
    b: u8,
}

impl From<Color> for QuantizedColor {
    fn from(color: Color) -> Self {
        let r = (color.r() * 255.0) as u8;
        let g = (color.g() * 255.0) as u8;
        let b = (color.b() * 255.0) as u8;
        Self { r, g, b }
    }
}

#[derive(Component)]
struct Ground {
    bitmap: Vec<Option<QuantizedColor>>,
    width: usize,
    height: usize,
}

impl Ground {
    fn bitmap_to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0; self.width * self.height * 4];
        for (i, pixel) in self.bitmap.iter().enumerate() {
            bytes[i * 4 + 3] = 255;
            if let Some(color) = pixel {
                bytes[i * 4 + 0] = color.r;
                bytes[i * 4 + 1] = color.g;
                bytes[i * 4 + 2] = color.b;
            }
        }
        bytes
    }
}

fn spawn_ground(mut commands: Commands, mut images: ResMut<Assets<Image>>, window: Query<&Window>) {
    let resolution = &window.single().resolution;
    let height = resolution.height() as usize;
    let width = resolution.width() as usize;
    let size = (height * width);
    let mut bitmap = vec![None; size];

    for row in 0..10 {
        for col in 0..width {
            bitmap[row * width + col] = Some(QuantizedColor {
                r: 255,
                g: 255,
                b: 255,
            });
        }
    }

    let ground = Ground {
        bitmap,
        width,
        height,
    };

    let ground_image = Image::new_fill(
        Extent3d {
            width: width as u32,
            height: height as u32,
            ..default()
        },
        TextureDimension::D2,
        &ground.bitmap_to_bytes(),
        TextureFormat::bevy_default(),
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    let ground_image_handle = images.add(ground_image);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width as f32, height as f32)),
                flip_y: true,
                ..default()
            },
            texture: ground_image_handle,
            ..default()
        })
        .insert((ground));
}

fn paint_ground(
    mut ground_query: Query<(&mut Ground, &Handle<Image>)>,
    mut images: ResMut<Assets<Image>>,
    balls_query: Query<(&Transform, &Velocity, &Sprite, Entity)>,
    mut commands: Commands,
    mut debug_info: ResMut<DebugInfo>,
) {
    let (mut ground, image_handle) = ground_query.single_mut();

    let width = ground.width;
    let height = ground.height;

    let image = images.get_mut(image_handle).unwrap();

    for (transform, velocity, sprite, entity) in &balls_query {
        let x = transform.translation.x + width as f32 / 2.0;
        let y = transform.translation.y + height as f32 / 2.0;
        if let Some(index) = within_bounds(x, y, width, height) {
            if ground.bitmap[index].is_some() {
                let mut y = y;
                let mut x = x;

                let normalized_velocity = velocity.linvel.normalize_or_zero();

                if normalized_velocity == Vec2::ZERO {
                    continue;
                }

                let dx = normalized_velocity.x;
                let dy = normalized_velocity.y;

                let mut maybe_index = within_bounds(x, y, width, height);

                while maybe_index.is_some() && ground.bitmap[maybe_index.unwrap()].is_some() {
                    y -= dy;
                    x -= dx;
                    maybe_index = within_bounds(x, y, width, height);
                }
                if let Some(index) = maybe_index {
                    let quantized_color = sprite.color.into();
                    ground.bitmap[index] = Some(quantized_color);
                    image.data[4 * index + 0] = quantized_color.r;
                    image.data[4 * index + 1] = quantized_color.g;
                    image.data[4 * index + 2] = quantized_color.b;
                }
                commands.entity(entity).despawn();
                debug_info.entity_count -= 1;
            }
        }
    }
}

fn within_bounds(x: f32, y: f32, width: usize, height: usize) -> Option<usize> {
    if (x as usize) < width && (y as usize) < height && x >= 0.0 && y >= 0.0 {
        Some(get_index(x, y, width))
    } else {
        None
    }
}

fn get_index(x: f32, y: f32, width: usize) -> usize {
    return (y as usize) * width + (x as usize);
}

fn calculate_mouse_position(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    window_query: Query<&Window>,
    mut mouse: ResMut<Mouse>,
    mut debug_info: ResMut<DebugInfo>,
) {
    let (camera_transform, camera) = camera_query.single();
    let window = window_query.single();

    let position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        .unwrap_or_default();

    mouse.position = position;
    debug_info.mouse_position = position;
}

const CONFETTI_SIZE: f32 = 1.0;

fn spread_joy(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
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
                Vec3::new(mouse.position.x, mouse.position.y, 0.0) + offset_vec_3 * 2.0,
            );

            commands
                .spawn(SpriteBundle {
                    transform,
                    sprite: Sprite {
                        color: random_color,
                        custom_size: Some(Vec2::splat(CONFETTI_SIZE)),
                        ..default()
                    },
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                // .insert(CollisionGroups::new(Group::NONE, Group::NONE))
                .insert(Collider::cuboid(CONFETTI_SIZE / 2.0, CONFETTI_SIZE / 2.0))
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
    let width = window.single().resolution.width();
    for (transform, entity) in &query {
        if transform.translation.y < -height / 2.0
            || transform.translation.y > height / 2.0
            || transform.translation.x < -width / 2.0
            || transform.translation.x > width / 2.0
        {
            debug_info.entity_count -= 1;
            commands.entity(entity).despawn();
        }
    }
}
