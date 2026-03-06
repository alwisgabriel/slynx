pub mod checker;
pub mod parser;
pub mod hir;
pub mod lexer;

// expose the primary type directly; users can still refer to the modules
// through the workspace root if necessary
pub use checker::TypeChecker;
