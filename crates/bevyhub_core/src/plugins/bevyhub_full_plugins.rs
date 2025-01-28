use crate::prelude::*;
use bevy::prelude::*;



pub fn bevyhub_full_plugins(app: &mut App) {
	app.add_plugins((
		BevyhubDefaultPlugins::with_bevyhub_assets(),
		DefaultPlaceholderPlugin,
		DefaultReplicatePlugin,
		UiTerminalPlugin,
	));
}
