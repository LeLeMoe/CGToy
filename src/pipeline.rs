use std::iter;
use winit::{dpi::PhysicalSize, window::Window};

pub struct PipelineState {
    size: PhysicalSize<u32>,
    surface: wgpu::Surface,
    sc_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
}

impl PipelineState {
    ///
    pub async fn new(window: &Window) -> Self {
        // Get window size
        let size = window.inner_size();
        // Create WGPU instance
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        // Create surface
        let surface = unsafe { instance.create_surface(window) };
        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("CGToy - Device(default)"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        // Configure surface
        let sc_format = surface.get_preferred_format(&adapter).unwrap();
        let sc_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: sc_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        surface.configure(&device, &sc_config);
        // Create shader module
        let shader_color = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("CGToy - Shader(color)"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/color.wgsl").into()),
        });
        // Create pipeline layout
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("CGToy - PipelineLayout(default)"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        // Create pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("CGToy - Pipeline(color)"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_color,
                entry_point: "main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                clamp_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_color,
                entry_point: "main",
                targets: &[sc_format.into()],
            }),
        });
        Self {
            size,
            surface,
            sc_config,
            device,
            queue,
            render_pipeline,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width != 0 && new_size.height != 0 {
            self.sc_config.width = new_size.width;
            self.sc_config.height = new_size.height;
            self.surface.configure(&self.device, &self.sc_config);
        }
    }

    pub fn render(&mut self) {
        // Get the current frame from swap chain
        let frame = match self.surface.get_current_frame() {
            Ok(frame) => frame.output,
            Err(wgpu::SurfaceError::Lost) => return self.resize(self.size),
            _ => return,
        };
        let frame_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        // Create command encoder
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("CGToy - Default Encoder"),
            });
        // Do clear render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("CGToy - ClearPass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }
        // Submit the commands
        self.queue.submit(iter::once(encoder.finish()));
    }
}
