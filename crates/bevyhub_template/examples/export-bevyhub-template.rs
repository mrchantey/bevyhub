//! a serializable version of the bevy 3d scene example
//! https://bevyengine.org/examples-webgpu/3d-rendering/3d-scene/
use bevyhub::prelude::*;
use bevy::prelude::*;

fn main() {
	SceneGroupExporter::new((register_types, DefaultReplicatePlugin))
		.add_scene("hello-world", || {})
		.add_scene("my-base-scene", spawn_simple_environment)
		.add_scene("my-beautiful-scene", spawn_simple_scene)
		.with_checks(DynamicSceneChecks::new().with_num_ignored_resources(11))
		.export_with_registries()
		.unwrap();
}

fn register_types(app: &mut App) {
	app.register_type::<Name>()
		.register_type::<Transform>()
		.register_type::<GlobalTransform>()
		.register_type::<BundlePlaceholder>();
}

pub fn spawn_simple_environment(mut commands: Commands) {
	// light
	commands.spawn((
		Name::new("Light"),
		BundlePlaceholder::PointLight,
		Transform::from_xyz(4.0, 8.0, 4.0),
	));
	// camera
	commands.spawn((
		Name::new("Camera"),
		BundlePlaceholder::Camera3d,
		Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
	));
}

pub fn spawn_simple_scene(mut commands: Commands) {
	// circular base
	commands.spawn((
		Name::new("Circle"),
		BundlePlaceholder::Pbr {
			mesh: Circle::new(4.0).into(),
			material: Color::WHITE.into(),
		},
		Transform::from_rotation(Quat::from_rotation_x(
			-std::f32::consts::FRAC_PI_2,
		)),
	));
	// cube
	commands.spawn((
		Name::new("Cube"),
		BundlePlaceholder::Pbr {
			mesh: Cuboid::new(1.0, 1.0, 1.0).into(),
			material: Color::srgb_u8(124, 144, 255).into(),
		},
		Transform::from_xyz(0.0, 0.5, 0.0),
	));
}
