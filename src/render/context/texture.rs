use uuid::Uuid;

///
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct TextureId(Uuid);

impl TextureId {
    ///
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Extent of a texture related operation.
pub struct Extent3d {
    pub width: u32,
    pub height: u32,
    pub depth_or_array_layers: u32,
}

impl From<Extent3d> for wgpu::Extent3d {
    fn from(extent_3d: Extent3d) -> Self {
        Self {
            width: extent_3d.width,
            height: extent_3d.height,
            depth_or_array_layers: extent_3d.depth_or_array_layers,
        }
    }
}

/// Dimensionality of a texture.
pub enum TextureDimension {
    /// 1D texture.
    D1,
    /// 2D texture.
    D2,
    /// 3D texture.
    D3,
}

impl From<TextureDimension> for wgpu::TextureDimension {
    fn from(dimension: TextureDimension) -> Self {
        match dimension {
            TextureDimension::D1 => Self::D1,
            TextureDimension::D2 => Self::D2,
            TextureDimension::D3 => Self::D3,
        }
    }
}

/// Underlying texture data format.
///
/// If there is a conversion in the format (such as srgb -> linear),
/// The conversion listed is for loading from texture in a shader.
/// When writing to the texture, the opposite conversion takes place.
pub enum TextureFormat {
    /// Red channel only. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    R8Unorm,
    /// Red channel only. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
    R8Snorm,
    /// Red channel only. 8 bit integer per channel. Unsigned in shader.
    R8Uint,
    /// Red channel only. 8 bit integer per channel. Signed in shader.
    R8Sint,
    /// Red channel only. 16 bit integer per channel. Unsigned in shader.
    R16Uint,
    /// Red channel only. 16 bit integer per channel. Signed in shader.
    R16Sint,
    /// Red channel only. 16 bit float per channel. Float in shader.
    R16Float,
    /// Red and green channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Rg8Unorm,
    /// Red and green channels. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
    Rg8Snorm,
    /// Red and green channels. 8 bit integer per channel. Unsigned in shader.
    Rg8Uint,
    /// Red and green channels. 8 bit integer per channel. Signed in shader.
    Rg8Sint,
    /// Red channel only. 32 bit integer per channel. Unsigned in shader.
    R32Uint,
    /// Red channel only. 32 bit integer per channel. Signed in shader.
    R32Sint,
    /// Red channel only. 32 bit float per channel. Float in shader.
    R32Float,
    /// Red and green channels. 16 bit integer per channel. Unsigned in shader.
    Rg16Uint,
    /// Red and green channels. 16 bit integer per channel. Signed in shader.
    Rg16Sint,
    /// Red and green channels. 16 bit float per channel. Float in shader.
    Rg16Float,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Rgba8Unorm,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    Rgba8UnormSrgb,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. [-127, 127] converted to/from float [-1, 1] in shader.
    Rgba8Snorm,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Unsigned in shader.
    Rgba8Uint,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Signed in shader.
    Rgba8Sint,
    /// Blue, green, red, and alpha channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Bgra8Unorm,
    /// Blue, green, red, and alpha channels. 8 bit integer per channel.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    Bgra8UnormSrgb,
    /// Red, green, blue, and alpha channels. 10 bit integer for RGB channels, 2 bit integer for alpha channel.
    /// [0, 1023] ([0, 3] for alpha) converted to/from float [0, 1] in shader.
    Rgb10a2Unorm,
    /// Red, green, and blue channels. 11 bit float with no sign bit for RG channels.
    /// 10 bit float with no sign bit for blue channel. Float in shader.
    Rg11b10Float,
    /// Red and green channels. 32 bit integer per channel. Unsigned in shader.
    Rg32Uint,
    /// Red and green channels. 32 bit integer per channel. Signed in shader.
    Rg32Sint,
    /// Red and green channels. 32 bit float per channel. Float in shader.
    Rg32Float,
    /// Red, green, blue, and alpha channels. 16 bit integer per channel. Unsigned in shader.
    Rgba16Uint,
    /// Red, green, blue, and alpha channels. 16 bit integer per channel. Signed in shader.
    Rgba16Sint,
    /// Red, green, blue, and alpha channels. 16 bit float per channel. Float in shader.
    Rgba16Float,
    /// Red, green, blue, and alpha channels. 32 bit integer per channel. Unsigned in shader.
    Rgba32Uint,
    /// Red, green, blue, and alpha channels. 32 bit integer per channel. Signed in shader.
    Rgba32Sint,
    /// Red, green, blue, and alpha channels. 32 bit float per channel. Float in shader.
    Rgba32Float,
    /// Special depth format with 32 bit floating point depth.
    Depth32Float,
    /// Special depth format with at least 24 bit integer depth.
    Depth24Plus,
    /// Special depth/stencil format with at least 24 bit integer depth and 8 bits integer stencil.
    Depth24PlusStencil8,
    /// Packed unsigned float with 9 bits mantisa for each RGB component, then a common 5 bits exponent
    Rgb9e5Ufloat,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 4 color + alpha pallet.
    /// 5 bit R + 6 bit G + 5 bit B + 1 bit alpha. [0, 63] ([0, 1] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc1RgbaUnorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px).
    /// 4 color + alpha pallet. 5 bit R + 6 bit G + 5 bit B + 1 bit alpha.
    /// Srgb-color [0, 63] ([0, 15] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc1RgbaUnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet.
    /// 5 bit R + 6 bit G + 5 bit B + 4 bit alpha. [0, 63] ([0, 15] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT3.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc2RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px).
    /// 4 color pallet. 5 bit R + 6 bit G + 5 bit B + 4 bit alpha.
    /// Srgb-color [0, 63] ([0, 255] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT3.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc2RgbaUnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet + 8 alpha pallet.
    /// 5 bit R + 6 bit G + 5 bit B + 8 bit alpha. [0, 63] ([0, 255] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT5.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc3RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px).
    /// 4 color pallet + 8 alpha pallet. 5 bit R + 6 bit G + 5 bit B + 8 bit alpha.
    /// Srgb-color [0, 63] ([0, 255] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT5.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc3RgbaUnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px).
    /// 8 color pallet. 8 bit R. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as RGTC1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc4RUnorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 8 color pallet. 8 bit R.
    /// [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// Also known as RGTC1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc4RSnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px).
    /// 8 color red pallet + 8 color green pallet. 8 bit RG. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as RGTC2.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc5RgUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 8 color red pallet + 8 color green pallet.
    /// 8 bit RG. [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// Also known as RGTC2.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc5RgSnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet.
    /// 16 bit unsigned float RGB. Float in shader.
    ///
    /// Also known as BPTC (float).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc6hRgbUfloat,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet.
    /// 16 bit signed float RGB. Float in shader.
    ///
    /// Also known as BPTC (float).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc6hRgbSfloat,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as BPTC (unorm).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc7RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as BPTC (unorm).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    Bc7RgbaUnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet.
    /// 8 bit integer RGB. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2RgbUnorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet.
    /// 8 bit integer RGB. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2RgbUnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet.
    /// 8 bit integer RGB + 1 bit alpha. [0, 255] ([0, 1] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2RgbA1Unorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px).
    /// Complex pallet. 8 bit integer RGB + 1 bit alpha.
    /// Srgb-color [0, 255] ([0, 1] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2RgbA1UnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet.
    /// 8 bit integer RGB + 8 bit alpha. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer RGB + 8 bit alpha.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format. 4x4 block compressed texture.
    /// 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer R. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacRUnorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet.
    /// 8 bit integer R. [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacRSnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet.
    /// 8 bit integer R + 8 bit integer G. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacRgUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet.
    /// 8 bit integer R + 8 bit integer G. [-127, 127] converted to/from float [-1, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacRgSnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc4x4RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc4x4RgbaUnormSrgb,
    /// 5x4 block compressed texture. 16 bytes per block (6.4 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc5x4RgbaUnorm,
    /// 5x4 block compressed texture. 16 bytes per block (6.4 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc5x4RgbaUnormSrgb,
    /// 5x5 block compressed texture. 16 bytes per block (5.12 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc5x5RgbaUnorm,
    /// 5x5 block compressed texture. 16 bytes per block (5.12 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc5x5RgbaUnormSrgb,
    /// 6x5 block compressed texture. 16 bytes per block (4.27 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc6x5RgbaUnorm,
    /// 6x5 block compressed texture. 16 bytes per block (4.27 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc6x5RgbaUnormSrgb,
    /// 6x6 block compressed texture. 16 bytes per block (3.56 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc6x6RgbaUnorm,
    /// 6x6 block compressed texture. 16 bytes per block (3.56 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc6x6RgbaUnormSrgb,
    /// 8x5 block compressed texture. 16 bytes per block (3.2 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc8x5RgbaUnorm,
    /// 8x5 block compressed texture. 16 bytes per block (3.2 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc8x5RgbaUnormSrgb,
    /// 8x6 block compressed texture. 16 bytes per block (2.67 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc8x6RgbaUnorm,
    /// 8x6 block compressed texture. 16 bytes per block (2.67 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc8x6RgbaUnormSrgb,
    /// 10x5 block compressed texture. 16 bytes per block (2.56 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc10x5RgbaUnorm,
    /// 10x5 block compressed texture. 16 bytes per block (2.56 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc10x5RgbaUnormSrgb,
    /// 10x6 block compressed texture. 16 bytes per block (2.13 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc10x6RgbaUnorm,
    /// 10x6 block compressed texture. 16 bytes per block (2.13 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc10x6RgbaUnormSrgb,
    /// 8x8 block compressed texture. 16 bytes per block (2 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc8x8RgbaUnorm,
    /// 8x8 block compressed texture. 16 bytes per block (2 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc8x8RgbaUnormSrgb,
    /// 10x8 block compressed texture. 16 bytes per block (1.6 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc10x8RgbaUnorm,
    /// 10x8 block compressed texture. 16 bytes per block (1.6 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc10x8RgbaUnormSrgb,
    /// 10x10 block compressed texture. 16 bytes per block (1.28 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc10x10RgbaUnorm,
    /// 10x10 block compressed texture. 16 bytes per block (1.28 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc10x10RgbaUnormSrgb,
    /// 12x10 block compressed texture. 16 bytes per block (1.07 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc12x10RgbaUnorm,
    /// 12x10 block compressed texture. 16 bytes per block (1.07 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc12x10RgbaUnormSrgb,
    /// 12x12 block compressed texture. 16 bytes per block (0.89 bit/px). Complex pallet.
    /// 8 bit integer RGBA. [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc12x12RgbaUnorm,
    /// 12x12 block compressed texture. 16 bytes per block (0.89 bit/px). Complex pallet.
    /// 8 bit integer RGBA. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_LDR`] must be enabled to use this texture format.
    Astc12x12RgbaUnormSrgb,
}

impl From<TextureFormat> for wgpu::TextureFormat {
    fn from(format: TextureFormat) -> Self {
        match format {
            TextureFormat::R8Unorm => Self::R8Unorm,
            TextureFormat::R8Snorm => Self::R8Snorm,
            TextureFormat::R8Uint => Self::R8Uint,
            TextureFormat::R8Sint => Self::R8Sint,
            TextureFormat::R16Uint => Self::R16Uint,
            TextureFormat::R16Sint => Self::R16Sint,
            TextureFormat::R16Float => Self::R16Float,
            TextureFormat::Rg8Unorm => Self::Rg8Unorm,
            TextureFormat::Rg8Snorm => Self::Rg8Snorm,
            TextureFormat::Rg8Uint => Self::Rg8Uint,
            TextureFormat::Rg8Sint => Self::Rg8Sint,
            TextureFormat::R32Uint => Self::R32Uint,
            TextureFormat::R32Sint => Self::R32Sint,
            TextureFormat::R32Float => Self::R32Float,
            TextureFormat::Rg16Uint => Self::Rg16Uint,
            TextureFormat::Rg16Sint => Self::Rg16Sint,
            TextureFormat::Rg16Float => Self::Rg16Float,
            TextureFormat::Rgba8Unorm => Self::Rgba8Unorm,
            TextureFormat::Rgba8UnormSrgb => Self::Rgba8UnormSrgb,
            TextureFormat::Rgba8Snorm => Self::Rgba8Snorm,
            TextureFormat::Rgba8Uint => Self::Rgba8Uint,
            TextureFormat::Rgba8Sint => Self::Rgba8Sint,
            TextureFormat::Bgra8Unorm => Self::Bgra8Unorm,
            TextureFormat::Bgra8UnormSrgb => Self::Bgra8UnormSrgb,
            TextureFormat::Rgb10a2Unorm => Self::Rgb10a2Unorm,
            TextureFormat::Rg11b10Float => Self::Rg11b10Float,
            TextureFormat::Rg32Uint => Self::Rg32Uint,
            TextureFormat::Rg32Sint => Self::Rg32Sint,
            TextureFormat::Rg32Float => Self::Rg32Float,
            TextureFormat::Rgba16Uint => Self::Rgba16Uint,
            TextureFormat::Rgba16Sint => Self::Rgba16Sint,
            TextureFormat::Rgba16Float => Self::Rgba16Float,
            TextureFormat::Rgba32Uint => Self::Rgba32Uint,
            TextureFormat::Rgba32Sint => Self::Rgba32Sint,
            TextureFormat::Rgba32Float => Self::Rgba32Float,
            TextureFormat::Depth32Float => Self::Depth32Float,
            TextureFormat::Depth24Plus => Self::Depth24Plus,
            TextureFormat::Depth24PlusStencil8 => Self::Depth24PlusStencil8,
            TextureFormat::Rgb9e5Ufloat => Self::Rgb9e5Ufloat,
            TextureFormat::Bc1RgbaUnorm => Self::Bc1RgbaUnorm,
            TextureFormat::Bc1RgbaUnormSrgb => Self::Bc1RgbaUnormSrgb,
            TextureFormat::Bc2RgbaUnorm => Self::Bc2RgbaUnorm,
            TextureFormat::Bc2RgbaUnormSrgb => Self::Bc2RgbaUnormSrgb,
            TextureFormat::Bc3RgbaUnorm => Self::Bc3RgbaUnorm,
            TextureFormat::Bc3RgbaUnormSrgb => Self::Bc3RgbaUnormSrgb,
            TextureFormat::Bc4RUnorm => Self::Bc4RUnorm,
            TextureFormat::Bc4RSnorm => Self::Bc4RSnorm,
            TextureFormat::Bc5RgUnorm => Self::Bc5RgUnorm,
            TextureFormat::Bc5RgSnorm => Self::Bc5RgSnorm,
            TextureFormat::Bc6hRgbUfloat => Self::Bc6hRgbUfloat,
            TextureFormat::Bc6hRgbSfloat => Self::Bc6hRgbSfloat,
            TextureFormat::Bc7RgbaUnorm => Self::Bc7RgbaUnorm,
            TextureFormat::Bc7RgbaUnormSrgb => Self::Bc7RgbaUnormSrgb,
            TextureFormat::Etc2RgbUnorm => Self::Etc2RgbUnorm,
            TextureFormat::Etc2RgbUnormSrgb => Self::Etc2RgbUnormSrgb,
            TextureFormat::Etc2RgbA1Unorm => Self::Etc2RgbA1Unorm,
            TextureFormat::Etc2RgbA1UnormSrgb => Self::Etc2RgbA1UnormSrgb,
            TextureFormat::EacRUnorm => Self::EacRUnorm,
            TextureFormat::EacRSnorm => Self::EacRSnorm,
            TextureFormat::EacRgUnorm => Self::EacRgUnorm,
            TextureFormat::EacRgSnorm => Self::EacRgSnorm,
            TextureFormat::Astc4x4RgbaUnorm => Self::Astc4x4RgbaUnorm,
            TextureFormat::Astc4x4RgbaUnormSrgb => Self::Astc4x4RgbaUnormSrgb,
            TextureFormat::Astc5x4RgbaUnorm => Self::Astc5x4RgbaUnorm,
            TextureFormat::Astc5x4RgbaUnormSrgb => Self::Astc5x4RgbaUnormSrgb,
            TextureFormat::Astc5x5RgbaUnorm => Self::Astc5x5RgbaUnorm,
            TextureFormat::Astc5x5RgbaUnormSrgb => Self::Astc5x5RgbaUnormSrgb,
            TextureFormat::Astc6x5RgbaUnorm => Self::Astc6x5RgbaUnorm,
            TextureFormat::Astc6x5RgbaUnormSrgb => Self::Astc6x5RgbaUnormSrgb,
            TextureFormat::Astc6x6RgbaUnorm => Self::Astc6x6RgbaUnorm,
            TextureFormat::Astc6x6RgbaUnormSrgb => Self::Astc6x6RgbaUnormSrgb,
            TextureFormat::Astc8x5RgbaUnorm => Self::Astc8x5RgbaUnorm,
            TextureFormat::Astc8x5RgbaUnormSrgb => Self::Astc8x5RgbaUnormSrgb,
            TextureFormat::Astc8x6RgbaUnorm => Self::Astc8x6RgbaUnorm,
            TextureFormat::Astc8x6RgbaUnormSrgb => Self::Astc8x6RgbaUnormSrgb,
            TextureFormat::Astc10x5RgbaUnorm => Self::Astc10x5RgbaUnorm,
            TextureFormat::Astc10x5RgbaUnormSrgb => Self::Astc10x5RgbaUnormSrgb,
            TextureFormat::Astc10x6RgbaUnorm => Self::Astc10x6RgbaUnorm,
            TextureFormat::Astc10x6RgbaUnormSrgb => Self::Astc10x6RgbaUnormSrgb,
            TextureFormat::Astc8x8RgbaUnorm => Self::Astc8x8RgbaUnorm,
            TextureFormat::Astc8x8RgbaUnormSrgb => Self::Astc8x8RgbaUnormSrgb,
            TextureFormat::Astc10x8RgbaUnorm => Self::Astc10x8RgbaUnorm,
            TextureFormat::Astc10x8RgbaUnormSrgb => Self::Astc10x8RgbaUnormSrgb,
            TextureFormat::Astc10x10RgbaUnorm => Self::Astc10x10RgbaUnorm,
            TextureFormat::Astc10x10RgbaUnormSrgb => Self::Astc10x10RgbaUnormSrgb,
            TextureFormat::Astc12x10RgbaUnorm => Self::Astc12x10RgbaUnorm,
            TextureFormat::Astc12x10RgbaUnormSrgb => Self::Astc12x10RgbaUnormSrgb,
            TextureFormat::Astc12x12RgbaUnorm => Self::Astc12x12RgbaUnorm,
            TextureFormat::Astc12x12RgbaUnormSrgb => Self::Astc12x12RgbaUnormSrgb,
        }
    }
}
