//! This is published as an app to bevyhub.dev
use bevyhub::prelude::*;
use bevyhub_core::scenes;
use bevy::prelude::*;

fn main() {
	App::new()
		.add_plugins((
			BevyhubDefaultPlugins::with_bevyhub_assets(),
			DefaultPlaceholderPlugin,
			DefaultReplicatePlugin,
			UiTerminalPlugin,
			temp_patches,
		))
		.add_systems(Startup, (scenes::camera_2d, scenes::ui_terminal_input))
		.run();
}
