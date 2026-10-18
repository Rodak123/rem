use std::path::Path;

use crate::SUPPORTED_EXTENSIONS;

/// Check if the current file is supported by Audio player
/// Has AppleDouble prevention
pub fn is_supported<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    let name_ok = path
        .file_name()
        .and_then(|n| n.to_str())
        // MacOS saves sometimes metadata into AppleDouble
        // file (don't ask me why it is a thing)
        .is_some_and(|n| !n.starts_with("._"));
    name_ok
        && path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| {
                SUPPORTED_EXTENSIONS
                    .iter()
                    .any(|s| s.eq_ignore_ascii_case(ext))
            })
}
