///
pub struct RenderGraph {}

impl RenderGraph {}

///
pub trait Pass {
    ///
    fn needed_features() -> wgpu::Features {
        wgpu::Features::empty()
    }
}
