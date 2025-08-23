#[cfg(all(feature = "colored", feature = "report_buf"))]
compile_error!("Cannot enable \"colored\" feature if \"report_buf\" feature is enabled");

#[cfg(not(feature = "sync"))]
pub(crate) type SharedStr = std::rc::Rc<str>;
#[cfg(feature = "sync")]
pub(crate) type SharedStr = std::sync::Arc<str>;

pub mod ast;
pub mod token;
pub mod scanner;
pub mod report;