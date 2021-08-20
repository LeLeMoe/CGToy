use super::resources::BufferID;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

///
pub struct RenderContext {
    instance: wgpu::Instance,
}

impl RenderContext {
    ///
    pub fn new() -> Self {
        todo!()
    }
}

///
struct Resources {
    buffers: Arc<RwLock<HashMap<BufferID, wgpu::Buffer>>>,

}
