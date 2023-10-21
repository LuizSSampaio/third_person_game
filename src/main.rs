use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use camera::CameraPlugin;
use floor::FloorPlugin;
use player::PlayerPlugin;
use point_light::PointLightPlugin;

mod camera;
mod floor;
mod player;
mod point_light;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CameraPlugin,
            FloorPlugin,
            PlayerPlugin,
            PointLightPlugin,
        ))
        .add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .run();
}
