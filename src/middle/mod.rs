pub mod typechecker;
pub mod optimizer;

pub use typechecker::TypeEnv;
pub use optimizer::optimize;