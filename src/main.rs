mod modules;

use bevy::prelude::*;
use modules::{player::PlayerPlugin, startup::StartupPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((PlayerPlugin, StartupPlugin))
        .run();
}
