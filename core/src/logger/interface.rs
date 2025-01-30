use tklog::{debug as tk_debug, error as tk_error, info as tk_info, warn as tk_warn};

pub fn info(text: &str) {
    tk_info!(text);
}

pub fn warn(text: &str) {
    tk_warn!(text);
}

pub fn fail(text: &str) { tk_error!(text); }

pub fn success(text: &str) {
    tk_info!(text);
}

pub fn script_log(text: &str) {
    tk_info!(text);
}

pub fn script_fail(text: &str) {
    tk_error!(text);
}

pub fn debug(text: &str) { tk_debug!(text); }
