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

macro_rules! wrap_in_thunk {
    // tt is a _little_ bit loose, but it'll do
    ($($code_block:tt)+) => {
        let thunk = || {
            $($code_block)+
        };
        thunk()
    };
}
pub(crate) use wrap_in_thunk;
