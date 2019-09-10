use std::cell::RefCell;
use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};
use texture_synthesis::{Session, SessionBuilder};
use uuid::Uuid;

use crate::DaliPipeline;
use crate::resource::{ResourceError, ResourceStorage};

/// Wraps a GlfwSurface, and initializes the Dali renderer
/// Use .pipeline() to start rendering
pub struct DaliContext {
    storage_loc: PathBuf,
}

impl DaliContext {
    /// Creates a new DaliContext
    pub fn new() -> DaliContext {
        let mut home = dirs::home_dir().unwrap();
        home.push("Dali");

        DaliContext { storage_loc: home }
    }

    pub fn resource(&mut self, name: &str) -> Result<ResourceStorage, ResourceError> {
        let mut path = self.storage_loc.clone();
        path.push("storage");
        path.push(name);
        ResourceStorage::new(path)
    }

    pub fn synthesize(&self) -> SessionBuilder {
        SessionBuilder::new()
    }

    /// Creates a new render pipeline
    pub fn pipeline(&mut self, (width, height): (u32, u32)) -> DaliPipeline<GlfwSurface> {
        let surface = GlfwSurface::new(
            WindowDim::Windowed(width, height),
            "Hello, world!",
            WindowOpt::default(),
        )
            .expect("GLFW surface creation");

        DaliPipeline::new(surface, [width, height])
    }
}
