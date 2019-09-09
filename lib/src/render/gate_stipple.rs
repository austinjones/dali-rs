use crate::render::semantics_stipple::VertexInstance;
use crate::stipple::Stipple;

/// Collects Stipple instances from the user, and provides an owned vec to LayerGate when the user has finished generating instances.
pub struct StippleGate {
    stipples: Vec<VertexInstance>,
}

impl StippleGate {
    pub fn new() -> StippleGate {
        StippleGate {
            stipples: Vec::new(),
        }
    }

    pub(crate) fn instances(self) -> Vec<VertexInstance> {
        self.stipples
    }

    pub fn draw(&mut self, stipple: &Stipple) {
        self.stipples.push(stipple.into());
    }
}
