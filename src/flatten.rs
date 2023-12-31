use std::ffi::{OsStr, OsString};

pub fn flatten_filename(i: &OsStr) -> OsString {
    if let Some(s) = i.to_str() {
        if let Some(s) = s.split('/').last() {
            return OsString::from(s);
        }
    }
    // fallback to source
    eprintln!("failed to flatten");
    return i.to_os_string();
}
