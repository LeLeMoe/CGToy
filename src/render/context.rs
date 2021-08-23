use super::resources::SamplerID;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use winit::{window::{Window, WindowId}, dpi::PhysicalSize};

///
#[derive(Clone)]
pub struct RenderContext {
    ctx_data: ContextSharedData,
    // resource: RenderContext,
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

        Self {
            ctx_data: ContextSharedData {
                instance: Arc::new(instance),
                adapter: Arc::new(adapter),
                device: Arc::new(device),
                queue: Arc::new(queue),
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
    surfaces: Arc<RwLock<HashMap<WindowId, wgpu::Surface>>>,
    samplers: Arc<RwLock<HashMap<SamplerID, wgpu::Sampler>>>,
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
            if self.data.adapter.is_surface_supported(&surface) {
                // Gets preferred format.
                let format = surface
                    .get_preferred_format(&self.ctx_data.adapter)
                    .unwrap();
                // Get window size.
                let size = window.inner_size();
                // Configures surface.
                surface.configure(
                    &self.ctx_data.device,
                    &wgpu::SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format,
                        width: size.width,
                        height: size.height,
                        present_mode: wgpu::PresentMode::Mailbox,
                    },
                );
                // Inserts it to the surfaces map.
                surfaces.insert(window_id, surface);
            } else {
                todo!("Throws an error that the adapter not support the surface.");
            }
        }
    }

    ///
    pub async fn update_surface(&self, id: WindowId, new_size: PhysicalSize<u32>) {
        // Gets the write lock.
        let mut surfaces = self.surfaces.write().await;
        //
    }

    ///
    pub async fn create_sampler(&self, desc: &wgpu::SamplerDescriptor) -> SamplerID {
        // Gets the write lock.
        let mut samplers = self.samplers.write().await;
        // Creates a new sampler id.
        let sampler_id = SamplerID::new();
        // Creates a new sampler.
        let sampler = self.ctx_data.device.create_sampler(desc);
        // Inserts it to samplers map.
        samplers.insert(sampler_id, sampler);
        sampler_id
    }

    ///
    pub async fn remove_sampler(&self, id: SamplerID) {
        // Gets the write lock.
        let mut samplers = self.samplers.write().await;
        // Remove target sampler from samplers map.
        samplers.remove(&id);
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
