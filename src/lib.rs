#[cfg(all(feature = "colored", feature = "report_buf"))]
compile_error!("Cannot enable \"colored\" feature if \"report_buf\" feature is enabled");

pub mod ast;
pub mod token;
pub mod scanner;
pub mod report;