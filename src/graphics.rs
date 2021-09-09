struct Graphics {
	vertex_buf: wgpu::Buffer,
	index_buf: wgpu::Buffer,
	index_count: usize,
	bind_group: wgpu::BindGroup,
	uniform_buf: wgpu::Buffer,
	pipeline: wgpu::RenderPipeline,
	pipeline_wire: Option<wgpu::RenderPipeline>,
}
