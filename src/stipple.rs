/// Represents a stippled brush stroke, with the parameters
/// - translation: [f32; 2] from -1 to 1
/// - scale: [f32; 2] from -`inf` to +`inf`: controls the size of the stipple bounds
/// - colormap_scale: [f32; 2] from -`inf` to +`inf`: controls the colormap rate of change.  large values produce more color variance
/// - rotation: [f32; 2] from -`inf` to +`inf` (radians)
/// - texture_rotation: f32 from -`inf` to +`inf` (radians): rotates the (optionally) bound texture, relative to the stipple location
/// - gamma: f32 from 0 to +`inf`: exponential gamma exposure, applied to the mask/alpha channel
#[derive(Clone, Debug)]
pub struct Stipple {
    pub(crate) translation: [f32; 2],
    pub(crate) scale: [f32; 2],
    pub(crate) colormap_scale: [f32; 2],
    pub(crate) rotation: f32,
    pub(crate) texture_rotation: f32,
    pub(crate) gamma: f32,
}

impl Stipple {
    pub fn new() -> Stipple {
        Self::default()
    }

    /// Translates the stipple, relative to the center of the canvas
    /// Range: [f32; 2] from -1 to 1
    pub fn with_translation(mut self, translation: [f32; 2]) -> Self {
        self.translation = translation;
        self
    }

    /// Scales the stipple, or applies a reflection if negative
    /// Range: [f32; 2] from -`inf` to +`inf`
    pub fn with_scale(mut self, scale: [f32; 2]) -> Self {
        self.scale = scale;
        self
    }

    /// Scales the colormap texture lookup, relative to the center of the stipple
    /// This can be used to generate texture when slightly below, or above 1.
    /// Range: [f32; 2] from -`inf` to +`inf`
    pub fn with_colormap_scale(mut self, scale: [f32; 2]) -> Self {
        self.colormap_scale = scale;
        self
    }

    /// Rotates the stipple by the given radians
    /// Range: [f32; 2] from -`inf` to +`inf` (radians)
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    /// If both a mask and texture are provided, the mask is fixed to the stipple, but the texture can be rotated.
    /// This can generate a lot of texture, as moire effects on unrelated images generate a lot of new texture.
    /// Range: f32 from -`inf` to +`inf` (radians)
    pub fn with_texture_rotation(mut self, texture_rotation: f32) -> Self {
        self.texture_rotation = texture_rotation;
        self
    }

    /// Applies exponential gamma to the mask (alpha channel)
    /// Range: f32 from 0 to +`inf`
    pub fn with_gamma(mut self, gamma: f32) -> Self {
        self.gamma = gamma;
        self
    }
}

impl Default for Stipple {
    fn default() -> Stipple {
        Stipple {
            translation: [0.0, 0.0],
            scale: [1.0, 1.0],
            colormap_scale: [1.0, 1.0],
            rotation: 0.0,
            texture_rotation: 0.0,
            gamma: 1.0,
        }
    }
}
