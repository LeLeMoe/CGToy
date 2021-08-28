use bitflags::bitflags;
use uuid::Uuid;

///
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct BufferId(Uuid);

impl BufferId {
    ///
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Describes a Buffer.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct BufferDescriptor {
    /// Size of a buffer.
    pub size: u64,
    /// Usages of a buffer. If the buffer is used in any way that isn’t specified here, the operation will panic.
    pub usage: BufferUsages,
    /// Allows a buffer to be mapped immediately after they are made.
    /// It does not have to be [`BufferUsages::MAP_READ`] or [`BufferUsages::MAP_WRITE`],
    /// all buffers are allowed to be mapped at creation.
    pub mapped_at_creation: bool,
}

impl From<&BufferDescriptor> for wgpu::BufferDescriptor<'_> {
    fn from(desc: &BufferDescriptor) -> Self {
        Self {
            label: None,
            size: desc.size,
            usage: desc.usage.into(),
            mapped_at_creation: false,
        }
    }
}

/// Describes a Buffer when allocating.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct BufferInitDescriptor<'a> {
    /// Contents of a buffer on creation.
    pub contents: &'a [u8],
    /// Usages of a buffer. If the buffer is used in any way that isn’t specified here, the operation will panic.
    pub usage: BufferUsages,
}

impl<'a> From<&BufferInitDescriptor<'a>> for wgpu::util::BufferInitDescriptor<'a> {
    fn from(desc: &BufferInitDescriptor<'a>) -> Self {
        Self {
            label: None,
            contents: desc.contents,
            usage: desc.usage.into(),
        }
    }
}

bitflags! {
    /// Different ways that you can use a buffer.
    ///
    /// The usages determine what kind of memory the buffer is allocated from and what actions the buffer can partake in.
    pub struct BufferUsages: u32 {
        /// Allow a buffer to be mapped for reading using [`Buffer::map_async`] + [`Buffer::get_mapped_range`].
        /// This does not include creating a buffer with [`BufferDescriptor::mapped_at_creation`] set.
        ///
        /// If [`Features::MAPPABLE_PRIMARY_BUFFERS`] isn't enabled, the only other usage a buffer
        /// may have is COPY_DST.
        const MAP_READ = 1 << 0;
        /// Allow a buffer to be mapped for writing using [`Buffer::map_async`] + [`Buffer::get_mapped_range_mut`].
        /// This does not include creating a buffer with `mapped_at_creation` set.
        ///
        /// If [`Features::MAPPABLE_PRIMARY_BUFFERS`] feature isn't enabled, the only other usage a buffer
        /// may have is COPY_SRC.
        const MAP_WRITE = 1 << 1;
        /// Allow a buffer to be the source buffer for a [`CommandEncoder::copy_buffer_to_buffer`] or
        /// [`CommandEncoder::copy_buffer_to_texture`] operation.
        const COPY_SRC = 1 << 2;
        /// Allow a buffer to be the destination buffer for a [`CommandEncoder::copy_buffer_to_buffer`],
        /// [`CommandEncoder::copy_texture_to_buffer`], [`CommandEncoder::fill_buffer`] or
        /// [`Queue::write_buffer`] operation.
        const COPY_DST = 1 << 3;
        /// Allow a buffer to be the index buffer in a draw operation.
        const INDEX = 1 << 4;
        /// Allow a buffer to be the vertex buffer in a draw operation.
        const VERTEX = 1 << 5;
        /// Allow a buffer to be a [`BufferBindingType::Uniform`] inside a bind group.
        const UNIFORM = 1 << 6;
        /// Allow a buffer to be a [`BufferBindingType::Storage`] inside a bind group.
        const STORAGE = 1 << 7;
        /// Allow a buffer to be the indirect buffer in an indirect draw call.
        const INDIRECT = 1 << 8;
    }
}

impl From<BufferUsages> for wgpu::BufferUsages {
    fn from(usages: BufferUsages) -> Self {
        Self::from_bits_truncate(usages.bits)
    }
}
