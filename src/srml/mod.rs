pub mod balances;
pub mod system;

#[derive(Debug)]
pub enum ModuleError {
    ModuleNotFound(&'static str),
    StorageNotFound(&'static str),
    StorageTypeError(&'static str),
}
