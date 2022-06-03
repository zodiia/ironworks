use bevy::{
	core_pipeline::Opaque3d,
	ecs::system::SystemParamItem,
	pbr::{
		DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
		SetMeshViewBindGroup,
	},
	prelude::*,
	reflect::TypeUuid,
	render::{
		mesh::MeshVertexBufferLayout,
		render_asset::{PrepareAssetError, RenderAsset, RenderAssetPlugin, RenderAssets},
		render_component::ExtractComponentPlugin,
		render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline},
		render_resource::{
			PipelineCache, RenderPipelineDescriptor, SpecializedMeshPipeline,
			SpecializedMeshPipelineError, SpecializedMeshPipelines,
		},
		view::ExtractedView,
		RenderApp, RenderStage,
	},
};

// TODO: should this be opaque in the long run?
type RenderMode = Opaque3d;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
	fn build(&self, app: &mut App) {
		app.add_asset::<Material>()
			.add_plugin(ExtractComponentPlugin::<Handle<Material>>::default())
			.add_plugin(RenderAssetPlugin::<Material>::default());

		app.sub_app_mut(RenderApp)
			.add_render_command::<RenderMode, Draw>()
			.init_resource::<Pipeline>()
			.init_resource::<SpecializedMeshPipelines<Pipeline>>()
			.add_system_to_stage(RenderStage::Queue, queue);
	}
}

// todo lmao
#[derive(Clone, Default, TypeUuid)]
#[uuid = "317a2fbb-6fb4-4bbd-b480-1d5942345cc0"]
pub struct Material;

impl RenderAsset for Material {
	type ExtractedAsset = Self;
	type PreparedAsset = Self;
	type Param = ();

	fn extract_asset(&self) -> Self::ExtractedAsset {
		self.clone()
	}

	fn prepare_asset(
		extracted_asset: Self::ExtractedAsset,
		_param: &mut SystemParamItem<Self::Param>,
	) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
		Ok(extracted_asset)
	}
}

// TODO: name
#[derive(Bundle, Default)]
pub struct MeshBundle {
	pub mesh: Handle<Mesh>,
	pub material: Handle<Material>,
	pub transform: Transform,
	pub global_transform: GlobalTransform,
	pub visibility: Visibility,
	pub computed_visibility: ComputedVisibility,
}

type Draw = (
	SetItemPipeline,
	SetMeshViewBindGroup<0>,
	// 	SetMaterialBindGroup<M, 1>,
	SetMeshBindGroup<1>,
	DrawMesh,
);

// TODO: seperate file
struct Pipeline {
	mesh_pipeline: MeshPipeline,
	vertex_shader: Handle<Shader>,
	fragment_shader: Handle<Shader>,
}

impl FromWorld for Pipeline {
	fn from_world(world: &mut World) -> Self {
		let asset_server = world.resource::<AssetServer>();
		let mesh_pipeline = world.resource::<MeshPipeline>();
		Pipeline {
			mesh_pipeline: mesh_pipeline.clone(),
			// TODO: at least the fragment shader should probably be from the material
			vertex_shader: asset_server.load("shader/mesh.wgsl"),
			fragment_shader: asset_server.load("shader/test.wgsl"),
		}
	}
}

impl SpecializedMeshPipeline for Pipeline {
	type Key = MeshPipelineKey;

	fn specialize(
		&self,
		key: Self::Key,
		layout: &MeshVertexBufferLayout,
	) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
		let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;

		descriptor.vertex.shader = self.vertex_shader.clone();

		let fragment = descriptor.fragment.as_mut().unwrap();
		fragment.shader = self.fragment_shader.clone();

		descriptor.layout = Some(vec![
			self.mesh_pipeline.view_layout.clone(),
			self.mesh_pipeline.mesh_layout.clone(),
		]);

		Ok(descriptor)
	}
}

#[allow(clippy::too_many_arguments)]
fn queue(
	draw_functions: Res<DrawFunctions<RenderMode>>,
	render_meshes: Res<RenderAssets<Mesh>>,
	pipeline: Res<Pipeline>,
	msaa: Res<Msaa>,
	mut pipelines: ResMut<SpecializedMeshPipelines<Pipeline>>,
	mut pipeline_cache: ResMut<PipelineCache>,
	material_meshes: Query<(Entity, &Handle<Mesh>, &MeshUniform, &Handle<Material>)>,
	mut views: Query<(&ExtractedView, &mut RenderPhase<RenderMode>)>,
) {
	let draw = draw_functions.read().get_id::<Draw>().unwrap();
	let msaa_key = MeshPipelineKey::from_msaa_samples(msaa.samples);
	// TODO: xref this impl with pbr's material, it looks like it's using the view to handle the entities
	for (view, mut phase) in views.iter_mut() {
		let view_matrix = view.transform.compute_matrix();
		let view_row_2 = view_matrix.row(2);

		for (entity, mesh_handle, mesh_uniform, _material) in material_meshes.iter() {
			// TODO: handle material
			// TODO: there's gotta be a clean way to get these without indenting everything like come on
			if let Some(mesh) = render_meshes.get(mesh_handle) {
				let key =
					msaa_key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
				let specialized_pipeline = pipelines
					.specialize(&mut pipeline_cache, &pipeline, key, &mesh.layout)
					.unwrap();

				phase.add(RenderMode {
					distance: view_row_2.dot(mesh_uniform.transform.col(3)),
					// fix naming on this so it's thingy
					pipeline: specialized_pipeline,
					entity,
					draw_function: draw,
				})
			}
		}
	}
}
