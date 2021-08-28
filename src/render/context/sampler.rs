use super::types::CompareFunction;
use std::num::NonZeroU8;
use uuid::Uuid;

///
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SamplerId(Uuid);

impl SamplerId {
    ///
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Describes a Sampler.
#[derive(Clone, Debug, PartialEq)]
pub struct SamplerDescriptor {
    /// How to deal with out of bounds accesses in the u (i.e. x) direction.
    pub address_mode_u: AddressMode,
    /// How to deal with out of bounds accesses in the v (i.e. y) direction.
    pub address_mode_v: AddressMode,
    /// How to deal with out of bounds accesses in the w (i.e. z) direction.
    pub address_mode_w: AddressMode,
    /// How to filter the texture when it needs to be magnified (made larger).
    pub mag_filter: FilterMode,
    /// How to filter the texture when it needs to be minified (made smaller).
    pub min_filter: FilterMode,
    /// How to filter between mip map levels.
    pub mipmap_filter: FilterMode,
    /// Minimum level of detail (i.e. mip level) to use.
    pub lod_min_clamp: f32,
    /// Maximum level of detail (i.e. mip level) to use.
    pub lod_max_clamp: f32,
    /// If this is enabled, this is a comparison sampler using the given comparison function.
    pub compare: Option<CompareFunction>,
    /// Valid values: 1, 2, 4, 8, and 16.
    pub anisotropy_clamp: Option<NonZeroU8>,
    /// Border color to use when address_mode is ['AddressMode::ClampToBorder']
    pub border_color: Option<SamplerBorderColor>,
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self {
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_filter: Default::default(),
            lod_min_clamp: 0.0,
            lod_max_clamp: f32::MAX,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        }
    }
}

impl From<&SamplerDescriptor> for wgpu::SamplerDescriptor<'_> {
    fn from(desc: &SamplerDescriptor) -> Self {
        Self {
            label: None,
            address_mode_u: desc.address_mode_u.into(),
            address_mode_v: desc.address_mode_v.into(),
            address_mode_w: desc.address_mode_w.into(),
            mag_filter: desc.mag_filter.into(),
            min_filter: desc.min_filter.into(),
            mipmap_filter: desc.mipmap_filter.into(),
            lod_min_clamp: desc.lod_min_clamp,
            lod_max_clamp: desc.lod_max_clamp,
            compare: desc.compare.map(|func| func.into()),
            anisotropy_clamp: desc.anisotropy_clamp,
            border_color: desc.border_color.map(|color| color.into()),
        }
    }
}

/// How edges should be handled in texture addressing.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum AddressMode {
    /// Clamp the value to the edge of the texture.
    ///
    /// -0.25 -> 0.0, 1.25 -> 1.0
    ClampToEdge,
    /// Repeat the texture in a tiling fashion
    ///
    /// -0.25 -> 0.75, 1.25 -> 0.25
    Repeat,
    /// Repeat the texture, mirroring it every repeat
    ///
    /// -0.25 -> 0.25, 1.25 -> 0.75
    MirrorRepeat,
    /// Clamp the value to the border of the texture Requires feature ['Features::ADDRESS_MODE_CLAMP_TO_BORDER'].
    ///
    /// -0.25 -> border, 1.25 -> border
    ClampToBorder,
}

impl Default for AddressMode {
    fn default() -> Self {
        Self::ClampToEdge
    }
}

impl From<AddressMode> for wgpu::AddressMode {
    fn from(mode: AddressMode) -> Self {
        match mode {
            AddressMode::ClampToEdge => Self::ClampToEdge,
            AddressMode::Repeat => Self::Repeat,
            AddressMode::MirrorRepeat => Self::MirrorRepeat,
            AddressMode::ClampToBorder => Self::ClampToBorder,
        }
    }
}

/// Texel mixing mode when sampling between texels.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum FilterMode {
    /// Nearest neighbor sampling.
    ///
    /// This creates a pixelated effect when used as a mag filter.
    Nearest,
    /// Linear Interpolation.
    ///
    /// This makes textures smooth but blurry when used as a mag filter.
    Linear,
}

impl Default for FilterMode {
    fn default() -> Self {
        Self::Nearest
    }
}

impl From<FilterMode> for wgpu::FilterMode {
    fn from(mode: FilterMode) -> Self {
        match mode {
            FilterMode::Nearest => Self::Nearest,
            FilterMode::Linear => Self::Linear,
        }
    }
}

/// Color variation to use when sampler addressing mode is ['AddressMode::ClampToBorder'].
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum SamplerBorderColor {
    /// RGBA: (0, 0, 0, 0).
    TransparentBlack,
    /// RGBA: (0, 0, 0, 1).
    OpaqueBlack,
    /// RGBA: (1, 1, 1, 1).
    OpaqueWhite,
}

impl From<SamplerBorderColor> for wgpu::SamplerBorderColor {
    fn from(color: SamplerBorderColor) -> Self {
        match color {
            SamplerBorderColor::TransparentBlack => Self::TransparentBlack,
            SamplerBorderColor::OpaqueBlack => Self::OpaqueBlack,
            SamplerBorderColor::OpaqueWhite => Self::OpaqueWhite,
        }
    }
}
