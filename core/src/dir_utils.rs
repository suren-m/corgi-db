use std::path::{Path, PathBuf};

pub fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let p = path_user_input.as_ref();
    if !p.starts_with("~") {
        return Some(p.to_path_buf());
    }
    if p == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut h| {
        if h == Path::new("/") {
            // Corner case: `h` root directory;
            // don't prepend extra `/`, just drop the tilde.
            p.strip_prefix("~").unwrap().to_path_buf()
        } else {
            h.push(p.strip_prefix("~/").unwrap());
            h
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expand_tilde() {
        // Should work on your linux box during tests, would fail in stranger
        // environments!
        let home = std::env::var("HOME").unwrap();
        let projects = PathBuf::from(format!("{}/Projects", home));
        assert_eq!(expand_tilde("~/Projects"), Some(projects));
        assert_eq!(expand_tilde("/foo/bar"), Some("/foo/bar".into()));
        assert_eq!(
            expand_tilde("~alice/projects"),
            Some("~alice/projects".into())
        );
    }
}
