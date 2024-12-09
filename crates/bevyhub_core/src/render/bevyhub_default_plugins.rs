use bevyhub_scene::utils::CliSceneLoadPlugin;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowPlugin;


const DEFAULT_ASSET_PATH: &str = "assets";

/// Opinionated [DefaultPlugins] to work well with scene-based workflows
/// and uploading to [bevyhub.org](https://bevyhub.org)
pub struct BevyhubDefaultPlugins {
	#[allow(unused)]
	pub native_asset_path: String,
	#[allow(unused)]
	pub wasm_asset_path: String,
	pub assert_local_assets: bool,
}

impl Default for BevyhubDefaultPlugins {
	fn default() -> Self {
		Self {
			native_asset_path: DEFAULT_ASSET_PATH.into(),
			wasm_asset_path: DEFAULT_ASSET_PATH.into(),
			assert_local_assets: false,
		}
	}
}




impl Plugin for BevyhubDefaultPlugins {
	fn build(&self, app: &mut App) {
		self.assert_local_assets();
		app.add_plugins((
			DefaultPlugins
				.set(bevyhub_window_plugin())
				.set(AssetPlugin {
					file_path: self.asset_path(),
					meta_check: AssetMetaCheck::Never,
					..default()
				})
				.build(),
			CliSceneLoadPlugin,
			// WorldInspectorPlugin::default()
			// .run_if(input_toggle_active(false, KeyCode::Tab)),
		))
		.add_systems(Update, close_on_esc);
	}
}


impl BevyhubDefaultPlugins {
	pub fn with_wasm_asset_path(wasm_asset_path: impl Into<String>) -> Self {
		Self {
			wasm_asset_path: wasm_asset_path.into(),
			..default()
		}
	}

	pub fn with_native_asset_path(path: impl Into<String>) -> Self {
		Self {
			native_asset_path: path.into(),
			..default()
		}
	}

	pub fn asset_path(&self) -> String {
		#[cfg(target_arch = "wasm32")]
		return self.wasm_asset_path.clone();
		#[cfg(not(target_arch = "wasm32"))]
		return self.native_asset_path.clone();
	}

	pub fn with_bevyhub_assets() -> Self {
		Self {
			wasm_asset_path:
				"https://bevyhub-public.s3.us-west-2.amazonaws.com/assets"
					.into(),
			// "https://storage.googleapis.com/beet-examples/assets".into(),
			assert_local_assets: true,
			..default()
		}
	}

	pub fn assert_local_assets(&self) {
		#[cfg(target_arch = "wasm32")]
		return;
		#[allow(unreachable_code)]
		if self.assert_local_assets
			&& !std::path::Path::new("assets/README.md").exists()
		{
			panic!(
				r#"
🥁🥁🥁
		
Welcome! Bevyhub examples use large assets that are stored remotely. 

Unix:

curl -o ./assets.tar.gz https://bevyhub-public.s3.us-west-2.amazonaws.com/assets.tar.gz
tar -xzvf ./assets.tar.gz
rm ./assets.tar.gz

Windows / manual:

1. Download https://bevyhub-public.s3.us-west-2.amazonaws.com/assets.tar.gz
2. Unzip into `./assets`

🥁🥁🥁
"#
			);
		}
	}
}

fn close_on_esc(
	mut commands: Commands,
	focused_windows: Query<(Entity, &Window)>,
	input: Res<ButtonInput<KeyCode>>,
) {
	for (window, focus) in focused_windows.iter() {
		if !focus.focused {
			continue;
		}

		if input.just_pressed(KeyCode::Escape) {
			commands.entity(window).despawn();
		}
	}
}



/// Ensure your app looks beautiful on bevyhub.org
pub fn bevyhub_window_plugin() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			fit_canvas_to_parent: true,
			canvas: Some("#bevyhub-canvas".to_string()),
			resizable: true,
			..default()
		}),
		..default()
	}
}
