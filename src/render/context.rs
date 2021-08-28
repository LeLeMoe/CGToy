use self::{
    buffer::{BufferDescriptor, BufferId, BufferInitDescriptor},
    sampler::{SamplerDescriptor, SamplerId},
    texture::TextureId,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalSize,
    window::{Window, WindowId},
};

pub mod buffer;
pub mod sampler;
pub mod texture;
pub mod types;

///
#[derive(Clone)]
pub struct RenderContext {
    ctx_data: ContextSharedData,
    resource: ResourceContext,
}

impl RenderContext {
    ///
    pub async fn new(desc: RenderContextDescriptor<'_>) -> Self {
        // Creates instance.
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        // Creates surface if window is provided.
        let surface = match desc.window {
            Some(window) => Some(unsafe { instance.create_surface(window) }),
            None => None,
        };
        // Requesst adapter.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: match surface {
                    Some(ref surface) => Some(surface),
                    None => None,
                },
            })
            .await
            .unwrap_or_else(|| panic!("Fail to request suitable adapter!"));
        // Requests device and queue.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: desc.features,
                    limits: Default::default(),
                },
                None,
            )
            .await
            .unwrap_or_else(|err| panic!("Fail to request device: {}", err));
        // Save context shared data
        let ctx_data = ContextSharedData {
            instance: Arc::new(instance),
            adapter: Arc::new(adapter),
            device: Arc::new(device),
            queue: Arc::new(queue),
        };

        Self {
            ctx_data: ctx_data.clone(),
            resource: ResourceContext {
                ctx_data,
                surfaces: Default::default(),
                samplers: Default::default(),
                buffers: Default::default(),
            },
        }
    }
}

///
pub struct RenderContextDescriptor<'a> {
    ///
    pub features: wgpu::Features,
    ///
    pub window: Option<&'a Window>,
}

///
#[derive(Clone)]
pub struct ResourceContext {
    ctx_data: ContextSharedData,
    surfaces: Arc<RwLock<HashMap<WindowId, (wgpu::Surface, wgpu::SurfaceConfiguration)>>>,
    samplers: Arc<RwLock<HashMap<SamplerId, wgpu::Sampler>>>,
    buffers: Arc<RwLock<HashMap<BufferId, wgpu::Buffer>>>,
}

impl ResourceContext {
    ///
    pub async fn create_surface(&self, window: &Window) {
        // Gets the window id.
        let window_id = window.id();
        // Gets the write lock.
        let mut surfaces = self.surfaces.write().await;
        // Checks if the surface has already been created.
        if !surfaces.contains_key(&window_id) {
            // Creates a new surface.
            let surface = unsafe { self.ctx_data.instance.create_surface(window) };
            // Checks if the new surface is suit for the adapter.
            if self.ctx_data.adapter.is_surface_supported(&surface) {
                // Gets preferred format.
                let format = surface
                    .get_preferred_format(&self.ctx_data.adapter)
                    .unwrap();
                // Gets window size.
                let size = window.inner_size();
                // Fills surface config desc.
                let desc = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format,
                    width: size.width,
                    height: size.height,
                    present_mode: wgpu::PresentMode::Mailbox,
                };
                // Configures surface.
                surface.configure(&self.ctx_data.device, &desc);
                // Inserts it to the surfaces map.
                surfaces.insert(window_id, (surface, desc));
            } else {
                todo!("Throws an error that the adapter not support the surface.");
            }
        }
    }

    ///
    pub async fn update_surface(&self, id: WindowId, new_size: PhysicalSize<u32>) {
        // Gets the write lock.
        let mut surfaces = self.surfaces.write().await;
        // Gets the target surface from the surfaces.
        if let Some((surface, desc)) = surfaces.get_mut(&id) {
            // Changes width and height in descriptor.
            desc.width = new_size.width;
            desc.height = new_size.height;
            // Reconfigures surfaces.
            surface.configure(&self.ctx_data.device, desc);
        }
    }

    ///
    pub async fn surface_next_frame(&self, id: WindowId) -> Option<wgpu::SurfaceFrame> {
        // Gets the read look.
        let surfaces = self.surfaces.read().await;
        // Gets the target surface from the surfaces.
        if let Some((surface, desc)) = surfaces.get(&id) {
            // Gets next frame and deal errors.
            match surface.get_current_frame() {
                // Success to get next frame.
                Ok(frame) => Some(frame),
                // Fail to get next frame.
                Err(error) => match error {
                    // Swap Chain has been lost and needs to be recreated.
                    wgpu::SurfaceError::Lost => {
                        surface.configure(&self.ctx_data.device, desc);
                        None
                    }
                    // No more memory left.
                    wgpu::SurfaceError::OutOfMemory => {
                        panic!("Fail to get frame from surfaces: {}", error);
                    }
                    // Timeout and outdated error should be dealt on next frame.
                    _ => None,
                },
            }
        } else {
            None
        }
    }

    ///
    pub async fn create_sampler(&self, desc: &SamplerDescriptor) -> SamplerId {
        // Gets the write lock.
        let mut samplers = self.samplers.write().await;
        // Creates a new sampler id.
        let sampler_id = SamplerId::new();
        // Creates a new sampler.
        let sampler = self.ctx_data.device.create_sampler(&desc.into());
        // Inserts it to samplers map.
        samplers.insert(sampler_id, sampler);
        sampler_id
    }

    ///
    pub async fn remove_sampler(&self, id: SamplerId) {
        // Gets the write lock.
        let mut samplers = self.samplers.write().await;
        // Remove target sampler from samplers map.
        samplers.remove(&id);
    }

    ///
    pub async fn create_buffer(&self, desc: &BufferDescriptor) -> BufferId {
        // Gets the write lock.
        let mut buffers = self.buffers.write().await;
        // Creates a new buffer id.
        let buffer_id = BufferId::new();
        // Creates a new buffer.
        let buffer = self.ctx_data.device.create_buffer(&desc.into());
        // Inserts it to samplers map.
        buffers.insert(buffer_id, buffer);
        buffer_id
    }

    ///
    pub async fn create_buffer_with_data(
        &self,
        desc: &BufferInitDescriptor<'_>,
    ) -> BufferId {
        // Gets the write lock.
        let mut buffers = self.buffers.write().await;
        // Creates a new buffer id.
        let buffer_id = BufferId::new();
        // Creates a new buffer.
        let buffer = self.ctx_data.device.create_buffer_init(&desc.into());
        // Inserts it to samplers map.
        buffers.insert(buffer_id, buffer);
        buffer_id
    }

    ///
    pub async fn remove_buffer(&self, id: BufferId) {
        // Gets the write lock.
        let mut buffers = self.buffers.write().await;
        // Remove target buffer from buffers map.
        buffers.remove(&id);
    }
}

///
#[derive(Clone)]
struct ContextSharedData {
    pub instance: Arc<wgpu::Instance>,
    pub adapter: Arc<wgpu::Adapter>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
}
