///
pub struct ResourceManager {}

impl ResourceManager {}

enum ResourceType {
    // Read only resources during a pass
    ShaderModule,       // Temp resource
    BindGroupLayout,
    PipelineLayout,
    RenderPipeline,
    ComputePipeline,
    // Both read and write resources during a pass
    Texture,
    Buffer,
    QuerySet,
}

///
pub trait CustomResource {}
