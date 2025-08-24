use bevy::{image::TextureAtlasLayout, input::keyboard, math::UVec2, prelude::*, sprite::Sprite};

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        // let adjustments: SpriteAdjustments = SpriteAdjustments { inputs: WASDInputs::default() };
        
        app.add_systems(Startup, setup_sprite);
        app.add_systems(Update, animate_sprite);
    }
}

// Indicies 
#[derive(Component)]
#[allow(dead_code)]
struct AnimationIndices {
    first: usize,
    last: usize
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

// Enum for setting the movement of the sprite.
#[derive(Component)]
enum Movement {
    Up, Down, 
    Left, Right, 
    UpLeft, UpRight,
    DownLeft, DownRight,
    None
}

// Vector2 for handling the sprite velocity.
#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32
}

// Animates the given sprite on the given timer.
fn animate_sprite(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,

    mut query: Query<(
        &mut AnimationIndices, 
        &mut AnimationTimer, 
        &mut Sprite, 
        &mut Movement,
        &mut Transform,
        &mut Velocity
    )>
) {
    for (mut indices, mut timer, mut sprite, mut logo, mut transform, mut velocity) in &mut query {
        timer.tick(time.delta());

        // Stories variables for movement logic.
        let w_pressed: bool = keyboard.pressed(KeyCode::KeyW);
        let a_pressed: bool = keyboard.pressed(KeyCode::KeyA);
        let s_pressed: bool = keyboard.pressed(KeyCode::KeyS);
        let d_pressed: bool = keyboard.pressed(KeyCode::KeyD);

        // Sets the direction of movement for the Direction enum, which allows
        // for the calculation of sprite velocity.
        *logo = Movement::None;

        // Horizontal directions.
        if a_pressed { *logo = Movement::Left; }
        if d_pressed { *logo = Movement::Right; }

        // Directional angle adjustments.
        if w_pressed { 
            *logo = Movement::Up;

            if a_pressed { *logo = Movement::UpLeft; }
            if d_pressed { *logo = Movement::UpRight; }
        }

        if s_pressed { 
            *logo = Movement::Down;

            if a_pressed { *logo = Movement::DownLeft; }
            if d_pressed { *logo = Movement::DownRight; }
        }

        // Sets the velocity of the character based on the input / movement keys.
        let walk_speed: f32 = 50.0;
        
        // Movement logic for horizontal velocity.
        if d_pressed || a_pressed {
            (*velocity).x = 
                if d_pressed {
                    1.0
                } else {
                    -1.0
                };
        } else {
            (*velocity).x = 0.0;
        }

        // Movement logic for vertical velocity.
        if w_pressed || s_pressed {
            (*velocity).y = 
                if w_pressed {
                    1.0
                } else {
                    -1.0
                };
        } else {
            (*velocity).y = 0.0;
        }

        // Apply the velocity to the sprite.
        transform.translation.x += (*velocity).x * walk_speed * time.delta_secs();
        transform.translation.y += (*velocity).y * walk_speed * time.delta_secs();

        // Set the animation indices for the next frame to render.
        match *logo {
            Movement::Down => { *indices = AnimationIndices { first: 0, last: 3 } }
            Movement::DownRight => { *indices = AnimationIndices { first: 4, last: 7 } },
            Movement::Right => { *indices = AnimationIndices { first: 8, last: 11 } }
            Movement::UpRight => { *indices = AnimationIndices { first: 12, last: 15 } },
            Movement::Up => { *indices = AnimationIndices { first: 16, last: 19 } }
            Movement::UpLeft => { *indices = AnimationIndices { first: 20, last: 23 } },
            Movement::Left => { *indices = AnimationIndices { first: 24, last: 27 } },
            Movement::DownLeft => { *indices = AnimationIndices { first: 28, last: 31 } },
            Movement::None => {}
        }

        // Set the atlas index to the first frame of the currently running animation.
        if let Some(atlas) = &mut sprite.texture_atlas {
            let offset: i32 = (indices.first as i32 + 3) - atlas.index as i32;
            if offset < 0 || offset > 3 {
                atlas.index = indices.first;
            }

            match* logo {
                Movement::None => {
                    // Sets the index to the following value.
                    atlas.index = indices.first;
                },

                Movement::Left | Movement::Right | Movement::Up | Movement::Down
                | Movement::UpLeft | Movement::UpRight | Movement::DownLeft | Movement::DownRight => {
                    // If the sprite animation timer has finished, then
                    // update the animation index to the next frame of the animation.
                    if timer.just_finished() {
                        // Sets the index to the following value.
                        atlas.index = 
                            if atlas.index == indices.last {
                                indices.first
                            } else {
                                atlas.index + 1
                            } 
                    }
                }
            }
        }
    }
}


// Initializes a sprite for setup.
fn setup_sprite(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) 
{
    commands.spawn(Camera2d);

    // The TextureAtlasLayout allows for the sprite to pull from a specific
    // section of the spritemap instead of rendering the entire image at once.
    let texture  = asset_server.load("char_walk.png");
    let layout: TextureAtlasLayout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 8, None, None);
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);

    let indices: AnimationIndices = AnimationIndices { first: 0, last: 3 };

    // Fetches the sprite from the atlas layout.
    let sprite = Sprite::from_atlas_image(
        texture, 
        TextureAtlas {
            layout: texture_atlas_layout,
            index: indices.first,
        }
    );

    // Spawn command for moving sprite.
    commands.spawn((
        sprite,

        Transform::from_scale(Vec3::splat(2.0)),
        indices,
        AnimationTimer(
            Timer::from_seconds(0.2, TimerMode::Repeating)
        ),
        Movement::None,
        Velocity { x: 0.0, y: 0.0 }
    ));
}