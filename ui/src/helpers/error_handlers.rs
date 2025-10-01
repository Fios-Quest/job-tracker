macro_rules! unwrap_or_report_and_return {
    ($thing_to_unwrap:expr) => {
        match $thing_to_unwrap {
            Ok(c) => c,
            Err(e) => {
                log::error!("{e}");
                *crate::EMIT_ERROR.write() = Some(format!("{e}"));
                return Default::default();
            }
        }
    };
}
pub(crate) use unwrap_or_report_and_return;

macro_rules! report_if_error {
    ($thing_to_unwrap:expr) => {
        match $thing_to_unwrap {
            Ok(_) => {}
            Err(e) => {
                log::error!("{e}");
                *crate::EMIT_ERROR.write() = Some(format!("{e}"));
            }
        }
    };
}
pub(crate) use report_if_error;

macro_rules! iife {
    // tt is a _little_ bit loose, but it'll do
    ($($code_block:tt)+) => {
        // Cheekily turning off clippy as in this case we need to do this for returns to work
        #[allow(clippy::redundant_closure_call)]
        (|| {
            $($code_block)+
        })()
    };
}
pub(crate) use iife;
