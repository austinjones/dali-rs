/// Represents a stippled brush stroke, with the parameters
/// - translation: [f32; 2] from -1 to 1
/// - scale: [f32; 2] from -`inf` to +`inf`: controls the size of the stipple bounds
/// - colormap_scale: [f32; 2] from -`inf` to +`inf`: controls the colormap rate of change.  large values produce more color variance
/// - rotation: [f32; 2] from -`inf` to +`inf` (radians)
#[derive(Clone, Debug)]
pub struct Stipple {
    pub(crate) translation: [f32; 2],
    pub(crate) scale: [f32; 2],
    pub(crate) colormap_scale: [f32; 2],
    pub(crate) rotation: f32,
    pub(crate) gamma: f32,
}

impl Stipple {
    pub fn new() -> Stipple {
        Self::default()
    }

    pub fn with_translation(mut self, translation: [f32; 2]) -> Self {
        self.translation = translation;
        self
    }

    pub fn with_scale(mut self, scale: [f32; 2]) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_colormap_scale(mut self, scale: [f32; 2]) -> Self {
        self.colormap_scale = scale;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

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
            gamma: 1.0,
        }
    }
}
