//! Compatibility re-exports for older `crate::pe::*` users.
//! New loader code should use `crate::loaders::pe::*`.
pub mod api_set_resolver;
pub use crate::loaders::pe::pe32;
pub use crate::loaders::pe::pe64;
pub use crate::loaders::pe::lief;
