mod render_graph;
mod resources;

///
struct RenderContext {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
}

impl RenderContext {
    /// Create a new render context.
    pub async fn new(desc: RenderContextDescriptor<'_>) -> Result<Self, RenderContextError> {
        // Create instance
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        // Create surface
        let surface = unsafe { instance.create_surface(desc.window) };
        // Request adapter
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
        // Check support features
        let supported_features = adapter.features();
        if !supported_features.contains(desc.features) {
            let mut unsupported_features = desc.features;
            unsupported_features.remove(supported_features);
            return Err(RenderContextError::FeaturesNotSupported(
                unsupported_features,
            ));
        }
        // Request device and queue
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
        // Get Surface size
        let size = desc.window.inner_size();
        // Fill swap chain descriptor
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        // Create swap chain
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Ok(Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
        })
    }

    ///
    pub fn update_swap_chain(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // Update size
        self.size = new_size;
        // Update swap chain size
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        // Create swap chain
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}

///
struct RenderContextDescriptor<'a> {
    pub label: Option<&'a str>,
    pub window: &'a winit::window::Window,
    pub features: wgpu::Features,
}

///
#[derive(Debug)]
enum RenderContextError {
    FailedToRequestAdapter,
    FailedToRequestDevice(wgpu::RequestDeviceError),
    FeaturesNotSupported(wgpu::Features),
}

