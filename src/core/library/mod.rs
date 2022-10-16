pub mod add;
pub mod cat;
pub mod dec;
pub mod div;
pub mod inc;
pub mod mul;
pub mod sub;

use super::module::Module;
use crate::interpreter::r#type::Type;

pub type CoreLibrary = Vec<Module>;
pub type Primitive = (String, Type);
pub type Primatives = Vec<Primitive>;
