use wgpu::*;
use winit::{dpi::*, window::*};

pub struct Renderer {
    device: Device,
    queue: Queue,
    surface_and_config: (Surface, SurfaceConfiguration),
}

impl Renderer {
    pub fn new(window: &Window, surface_size: Option<PhysicalSize<u32>>) -> Self {
        // create wgpu instance
        let instance = Instance::new(Backends::all());
        // create surface for window
        let surface = unsafe { instance.create_surface(window) };
        // get gpu handle
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
        }))
        // get gpu device
        .expect("Failed to find an appropriate adapter");
        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: Limits::downlevel_defaults().using_resolution(adapter.limits()),
            },
            None,
        ))
        .expect("Failed to create device");
        // configure surface
        let size = surface_size.unwrap_or(window.inner_size());
        let swapchain_format = surface.get_preferred_format(&adapter).unwrap();
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);
        Self {
            device,
            queue,
            surface_and_config: (surface, surface_config),
        }
    }
    pub fn set_surface_size(&mut self, surface_size: PhysicalSize<u32>) {
        let (surface, config) = &mut self.surface_and_config;
        config.width = surface_size.width;
        config.height = surface_size.height;
        surface.configure(&self.device, &config);
    }
    pub fn load_shader_from_memory(&self, shader: &'static str) -> ShaderModule {
        self.device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(shader.into()),
        })
    }
    pub fn create_pipeline_layout(
        &self,
        bind_group_layouts: &[&BindGroupLayout],
    ) -> PipelineLayout {
        self.device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            })
    }
    pub fn create_render_pipeline(
        &self,
        pipeline_layout: &PipelineLayout,
        shader_module: &ShaderModule,
    ) -> RenderPipeline {
        let (_, surface_config) = &self.surface_and_config;
        self.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(FragmentState {
                    module: &shader_module,
                    entry_point: "fs_main",
                    targets: &[surface_config.format.into()],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    polygon_mode: PolygonMode::Fill,
                    clamp_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState::default(),
            })
    }
    pub fn render_pass<F>(&mut self, clear_color: Color, f: F)
    where
        F: FnOnce(&mut RenderPass),
    {
        let (surface, _) = &self.surface_and_config;
        let frame = surface
            .get_current_frame()
            .expect("Failed to acquire next swapchain texture")
            .output;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(clear_color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            f(&mut render_pass);
        }
        self.queue.submit(Some(encoder.finish()));
    }
}
