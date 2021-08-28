/// Comparison function used for depth and stencil operations.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum CompareFunction {
    /// Function never passes.
    Never,
    /// Function passes if new value less than existing value.
    Less,
    /// Function passes if new value is equal to existing value.
    Equal,
    /// Function passes if new value is less than or equal to existing value.
    LessEqual,
    /// Function passes if new value is greater than existing value.
    Greater,
    /// Function passes if new value is not equal to existing value.
    NotEqual,
    /// Function passes if new value is greater than or equal to existing value.
    GreaterEqual,
    /// Function always passes.
    Always,
}

impl From<CompareFunction> for wgpu::CompareFunction {
    fn from(func: CompareFunction) -> Self {
        match func {
            CompareFunction::Never => Self::Never,
            CompareFunction::Less => Self::Less,
            CompareFunction::Equal => Self::Equal,
            CompareFunction::LessEqual => Self::LessEqual,
            CompareFunction::Greater => Self::Greater,
            CompareFunction::NotEqual => Self::NotEqual,
            CompareFunction::GreaterEqual => Self::Never,
            CompareFunction::Always => Self::Always,
        }
    }
}