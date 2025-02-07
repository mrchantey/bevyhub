use anyhow::Result;
use bevyhub_net::prelude::*;
use bevyhub_scene::prelude::*;
use bevy::app::Plugins;
use bevy::ecs::query::QueryFilter;


/// Export all required elements of your app,
/// 1. [SceneGroupExporter]
/// 2. [TypeRegistryExporter]
/// 3. [ReplicateRegistryExporter]
pub struct BevyhubExporter<Plugin, PluginMarker, QueryFilter> {
	pub scene_group_exporter:
		SceneGroupExporter<Plugin, PluginMarker, QueryFilter>,
	pub type_registry: TypeRegistryExporter<Plugin, PluginMarker>,
	pub replicate_registry: ReplicateRegistryExporter<Plugin, PluginMarker>,
}

impl<P: Clone + Plugins<M>, M, Q: QueryFilter> BevyhubExporter<P, M, Q> {
	pub fn new(plugin: P) -> Self {
		Self {
			scene_group_exporter: SceneGroupExporter::new(plugin.clone())
				.with_filter::<Q>(),
			type_registry: TypeRegistryExporter::new(plugin.clone()),
			replicate_registry: ReplicateRegistryExporter::new(plugin),
		}
	}
	pub fn export(self) -> Result<()> {
		self.scene_group_exporter.export()?;
		self.type_registry.export()?;
		self.replicate_registry.export()?;
		Ok(())
	}
}


#[extend::ext(name=SceneGroupExporterrExt)]
pub impl<P: Clone + Plugins<M>, M, Q: QueryFilter> SceneGroupExporter<P, M, Q> {
	fn export_with_registries(self) -> Result<()> {
		self.export_with_registries_and_options("target/registries")
	}
	fn export_with_registries_and_options(
		self,
		registries_dir: &str,
	) -> Result<()> {
		TypeRegistryExporter::new(self.plugin.clone())
			.with_dir(registries_dir)
			.export()?;

		ReplicateRegistryExporter::new(self.plugin.clone())
			.with_dir(registries_dir)
			.export()?;

		self.export()?;
		Ok(())
	}
}
