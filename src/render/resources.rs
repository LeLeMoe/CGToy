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





