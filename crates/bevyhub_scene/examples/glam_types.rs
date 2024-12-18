use bevy::prelude::*;
use bevy_reflect::TypeRegistry;

fn main() {
	let vec = Vec3::new(0., 1., 2.);
	let str = serde_json::to_string(&vec).unwrap();
	// serializes to a list
	println!("{}", str); // [0.0, 1.0, 2.0]

	let mut registry = TypeRegistry::default();
	registry.register::<Vec3>();
	let type_info = registry
		.get_type_info(std::any::TypeId::of::<Vec3>())
		.unwrap();
	// but the type is a struct
	assert!(type_info.as_struct().is_ok());
}
