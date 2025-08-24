#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod bev { include!("bev.rs"); }

use bevy::prelude::*;
use bev::hello_plugin::{HelloPlugin};
use bev::sprite_plugin::{SpritePlugin};

use crate::bev::shape_plugin::ShapePlugin;

fn main() {
    println!("Launching app...");
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest() // Removes blurry images (Nearest Neighbor scaling)
        )) // Adds first-party rendering plugins for Bevy.
        .add_plugins(SpritePlugin)
                
        // Runs the combination of the added systems.
        .run();
}
