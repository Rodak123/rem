use std::{borrow::Cow, path::Path};

/// Takes '~' from start of the path as Home directory
pub fn expand_tilde<P: AsRef<Path>>(path: &P) -> Cow<'_, Path> {
    let path = path.as_ref();
    if let Ok(rest) = path.strip_prefix("~")
        && let Some(home) = dirs::home_dir()
    {
        return Cow::Owned(home.join(rest));
    }
    Cow::Borrowed(path)
}
