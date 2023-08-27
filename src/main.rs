mod modules;

use bevy::prelude::*;
use modules::{player::PlayerPlugin, scene::ScenePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, ScenePlugin))
        .run();
}
