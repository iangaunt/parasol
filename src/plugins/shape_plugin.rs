use bevy::color::Color;
use bevy::input::keyboard::KeyCode;
use bevy::math::primitives::{Annulus, Circle, RegularPolygon};
use bevy::prelude::*;
use bevy::sprite::{MeshMaterial2d, Wireframe2dConfig, Wireframe2dPlugin};
use bevy::transform::components::Transform;

// Creation of a plugin for handling rendering shapes.
pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
       app.add_plugins(Wireframe2dPlugin::default());
       app.add_systems(Startup, setup_shapes);
       app.add_systems(Update, wireframe);
    }
}

// Initializes the shape meshes to be added to the application.
fn setup_shapes(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>) 
{
    // Creates a new Camera2d entity for rendering.
    commands.spawn(Camera2d);

    // Creates a lit of shape meshes for 2D-rendering.
    let shapes = [
        meshes.add(Circle::new(25.0)),
        meshes.add(Ellipse::new(25.0, 40.0)),
        meshes.add(Annulus::new(25.0, 40.0)),
        meshes.add(Rectangle::new(50.0, 80.0)),
        meshes.add(RegularPolygon::new(40.0, 10)),
        meshes.add(Triangle2d::new(Vec2::Y * 40.0, Vec2::new(-40.0, -40.0), Vec2::new(40.0, -40.0)))
    ];
    let shape_count = shapes.len();
    const X_DISPLAY: f32 = 500.0;

    // Iterates over the shapes in the `shapes` list and applies a 
    // special color material depending on the index of the shape 
    // in the shapes list.
    for (i, shape) in shapes.into_iter().enumerate() {
        // Initialize ColorMaterial.
        let color: Color = Color::hsl(360.0 * i as f32 / shape_count as f32, 0.5, 0.5);
        let x_pos: f32 = -X_DISPLAY / 2.0 + i as f32 / (shape_count - 1) as f32 * X_DISPLAY;

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(x_pos, 0.0, 0.0)
        ));
    }
}

// Toggles the global wireframe tag for 2d rendering,
// allowing the user to switch between wireframe and solid.
fn wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}