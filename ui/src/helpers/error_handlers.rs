macro_rules! unwrap_error_or_report_and_return {
    ($thing_to_unwrap:expr) => {
        match $thing_to_unwrap {
            Ok(c) => c,
            Err(e) => {
                log::error!("{e}");
                *crate::EMIT_ERROR.write() = Some(format!("{e}"));
                return;
            }
        }
    };
}
pub(crate) use unwrap_error_or_report_and_return;
