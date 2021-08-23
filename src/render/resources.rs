use uuid::Uuid;

///
pub struct ResourceManager {}

impl ResourceManager {}

enum ResourceType {
    // Read only resources during a pass
    ShaderModule,
    BindGroupLayout,
    BindGroup,
    PipelineLayout,
    RenderPipeline,
    ComputePipeline,
    // Both read and write resources during a pass
    Texture,
    Buffer,
    QuerySet, // Not supported for now
}

///
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct BufferID(Uuid);

impl BufferID {
    ///
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

///
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct TextureID(Uuid);

impl TextureID {
    ///
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

///
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct SamplerID(Uuid);

impl SamplerID {
    ///
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
