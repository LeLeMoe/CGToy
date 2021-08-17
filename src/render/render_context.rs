use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window};

///
pub struct RenderContext {
    size: PhysicalSize<u32>,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    accessor: ContextAccessor,
}

impl RenderContext {
    ///
    pub async fn new(desc: RenderContextDescriptor<'_>) -> Result<Self, RenderContextError> {
        // Create instance.
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        // Create surface.
        let surface = unsafe { instance.create_surface(desc.window) };
        // Request adapter.
        let adapter = match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
        {
            Some(adapter_res) => adapter_res,
            None => return Err(RenderContextError::FailedToRequestAdapter),
        };
        // Check support features.
        let supported_features = adapter.features();
        if !supported_features.contains(desc.features) {
            let mut unsupported_features = desc.features;
            unsupported_features.remove(supported_features);
            return Err(RenderContextError::FeaturesNotSupported(
                unsupported_features,
            ));
        }
        // Request device and queue.
        let (device, queue) = match adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: desc.features,
                    limits: Default::default(),
                },
                None,
            )
            .await
        {
            Ok(device_bundle) => device_bundle,
            Err(err) => return Err(RenderContextError::FailedToRequestDevice(err)),
        };
        // Get Surface size.
        let size = desc.window.inner_size();
        // Fill swap chain descriptor.
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        // Create swap chain.
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Ok(Self {
            size,
            sc_desc,
            swap_chain,
            accessor: ContextAccessor(Arc::new(ContextData {
                surface,
                device,
                queue,
            })),
        })
    }

    ///
    pub fn access(&self) -> ContextAccessor {
        ContextAccessor(self.accessor.0.clone())
    }

    ///
    pub fn update_swap_chain(&mut self, new_size: PhysicalSize<u32>) {
        if self.size.width > 0 && self.size.height > 0 {
            self.size = new_size;
            // Update swap chain size.
            self.sc_desc.width = new_size.width;
            self.sc_desc.height = new_size.height;
            // Create swap chain.
            self.swap_chain = self
                .accessor
                .0
                .device
                .create_swap_chain(&self.accessor.0.surface, &self.sc_desc);
        }
    }

    ///
    pub fn get_current_frame(&mut self) -> Option<wgpu::SwapChainTexture> {
        match self.swap_chain.get_current_frame() {
            Ok(frame) => Some(frame.output),
            Err(error) => {
                match error {
                    wgpu::SwapChainError::Lost => self.update_swap_chain(self.size),
                    wgpu::SwapChainError::OutOfMemory => panic!("Out of memory!"),
                    _ => (),
                };
                None
            }
        }
    }
}

///
pub struct RenderContextDescriptor<'a> {
    pub window: &'a Window,
    pub features: wgpu::Features,
}

///
pub enum RenderContextError {
    FailedToRequestAdapter,
    FailedToRequestDevice(wgpu::RequestDeviceError),
    FeaturesNotSupported(wgpu::Features),
}

///
pub struct ContextAccessor(Arc<ContextData>);

impl ContextAccessor {
    ///
    pub fn create_shader_module(&self, desc: &wgpu::ShaderModuleDescriptor) -> wgpu::ShaderModule {
        self.0.device.create_shader_module(desc)
    }

    ///
    pub fn create_command_encoder(
        &self,
        desc: &wgpu::CommandEncoderDescriptor,
    ) -> wgpu::CommandEncoder {
        self.0.device.create_command_encoder(desc)
    }

    ///
    pub fn create_bind_group(&self, desc: &wgpu::BindGroupDescriptor) -> wgpu::BindGroup {
        self.0.device.create_bind_group(desc)
    }

    ///
    pub fn create_bind_group_layout(
        &self,
        desc: &wgpu::BindGroupLayoutDescriptor,
    ) -> wgpu::BindGroupLayout {
        self.0.device.create_bind_group_layout(desc)
    }

    ///
    pub fn create_pipeline_layout(
        &self,
        desc: &wgpu::PipelineLayoutDescriptor,
    ) -> wgpu::PipelineLayout {
        self.0.device.create_pipeline_layout(desc)
    }

    ///
    pub fn create_render_pipeline(
        &self,
        desc: &wgpu::RenderPipelineDescriptor,
    ) -> wgpu::RenderPipeline {
        self.0.device.create_render_pipeline(desc)
    }

    ///
    pub fn create_compute_pipeline(
        &self,
        desc: &wgpu::ComputePipelineDescriptor,
    ) -> wgpu::ComputePipeline {
        self.0.device.create_compute_pipeline(desc)
    }

    ///
    pub fn create_buffer(&self, desc: &wgpu::BufferDescriptor) -> wgpu::Buffer {
        self.0.device.create_buffer(desc)
    }

    ///
    pub fn create_buffer_with_data(&self, usage: wgpu::BufferUsage, data: &[u8]) -> wgpu::Buffer {
        self.0
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: data,
                usage,
            })
    }

    ///
    pub fn create_texture(&self, desc: &wgpu::TextureDescriptor) -> wgpu::Texture {
        self.0.device.create_texture(desc)
    }

    ///
    pub fn create_texture_with_data(
        &self,
        desc: &wgpu::TextureDescriptor,
        data: &[u8],
    ) -> wgpu::Texture {
        self.0
            .device
            .create_texture_with_data(&self.0.queue, desc, data)
    }

    ///
    pub fn create_sampler(&self, desc: &wgpu::SamplerDescriptor) -> wgpu::Sampler {
        self.0.device.create_sampler(desc)
    }

    ///
    pub fn write_buffer(&self, buffer: &wgpu::Buffer, offset: wgpu::BufferAddress, data: &[u8]) {
        self.0.queue.write_buffer(buffer, offset, data);
    }

    ///
    pub fn write_texture(
        &self,
        texture: wgpu::ImageCopyTexture<'_>,
        data_layout: wgpu::ImageDataLayout,
        size: wgpu::Extent3d,
        data: &[u8],
    ) {
        self.0.queue.write_texture(texture, data, data_layout, size);
    }

    ///
    pub fn submit<I: IntoIterator<Item = wgpu::CommandBuffer>>(&self, cmd_buffer: I) {
        self.0.queue.submit(cmd_buffer);
    }
}

///
struct ContextData {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}
