// WARNING: generated file - manual changes will be overriden

#[allow(unused_imports)]
pub mod prelude {

    pub use super::super::inferenceobjectives::*;
    pub use super::super::inferencepools::*;

    pub use super::super::common::*;
}
use prelude::*;

impl Default for InferencePoolExtensionRefFailureMode {
    fn default() -> Self {
        InferencePoolExtensionRefFailureMode::FailOpen
    }
}
