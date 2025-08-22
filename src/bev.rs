// Imports plugins + resources from outside files.
pub mod hello_plugin { include!("plugins/hello_plugin.rs"); }
pub mod shape_plugin { include!("plugins/shape_plugin.rs"); }
pub mod sprite_plugin { include!("plugins/sprite_plugin.rs"); }