use dali::DaliContext;

/// GlfwSurface must be initialized from the main thread.
/// Unfortuantely, Rust tests cannot be forced to run on the main thread
/// So this workaround tests that the Context can be created
pub fn main() {
    let context = DaliContext::new();
}