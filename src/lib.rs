pub mod schema;
pub mod serde;

pub use schema::*;
pub use serde::*;

// compile tests
#[cfg(test)]
#[path = "../tests/ClsSchemas.rs"]
mod test_cls_schemas;
#[cfg(test)]
#[path = "../tests/DespatchSchemas.rs"]
mod test_despatch_schemas;
// #[cfg(test)]
// #[path = "../tests/OwnSchemas.rs"]
// mod test_own_schemas;
