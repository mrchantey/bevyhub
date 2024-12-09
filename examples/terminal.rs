//! This is published as an app to bevyhub.org
use bevyhub::prelude::*;
use bevyhub_core::scenes;
use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins(bevyhub_full_plugins)
		.add_systems(Startup, (scenes::camera_2d, scenes::ui_terminal_input))
		.run();
}
