use super::module::Module;

pub mod add;
pub mod cat;
pub mod dec;
pub mod div;
pub mod inc;
pub mod mul;
pub mod sub;

pub type CoreLibrary = Vec<Module>;
