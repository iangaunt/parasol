use bevy::{image::TextureAtlasLayout, math::UVec2, prelude::*, sprite::Sprite};

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sprite);
        app.add_systems(Update, animate_sprite);
    }
}

#[derive(Component)]
#[allow(dead_code)]
struct AnimationIndices {
    first: usize,
    last: usize
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

// Animates the given sprite on the given timer.
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        
        // If the sprite animation timer has finished, then
        // update the animation index to the next frame of the animation.
        if timer.just_finished() {
            // Checks to see if the current frame exists (prevents out of bounds).
            if let Some(atlas) = &mut sprite.texture_atlas {
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

    commands.spawn((
        // Fetches the sprite from the atlas layout.
        Sprite::from_atlas_image(
            texture, 
            TextureAtlas {
                layout: texture_atlas_layout,
                index: indices.first,
            }
        ),

        Transform::from_scale(Vec3::splat(3.0)),
        indices,
        AnimationTimer(
            Timer::from_seconds(0.3, TimerMode::Repeating)
        )
    ));
}