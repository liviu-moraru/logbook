//! Records observations in a logbook file, or lists previous
//! observations.

use anyhow::Result;
use std::fs::{File, exists, read_to_string};
use std::io::Write;
use std::path::Path;

/// Reads the contents of the logbook file at `path`.
///
/// Returns [`None`] if the file does not exist or is empty.
///
/// # Errors
///
/// Returns any error from [`exists`] or [`read_to_string`].
pub fn read(path: impl AsRef<Path>) -> Result<Option<String>> {
    if exists(&path)? {
        let text = read_to_string(path)?;
        if text.is_empty() {
            Ok(None)
        } else {
            Ok(Some(text))
        }
    } else {
        Ok(None)
    }
}

/// Appends `msg` to the logbook file at `path`, creating the file if necessary.
///
/// # Errors
///
/// Returns any error from [`open`](std::fs::OpenOptions::open) or [`writeln!`].
pub fn append(path: impl AsRef<Path>, message: &str) -> Result<()> {
    let mut file = File::options().create(true).append(true).open(path)?;
    writeln!(file, "{message}")?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_returns_none_if_file_does_not_exist() {
        let got = read("tests/data/bogus.txt").unwrap();
        assert_eq!(got, None, "expected None");
    }

    #[test]
    fn read_returns_none_for_empty_file() {
        let got = read("tests/data/empty.txt").unwrap();
        assert_eq!(got, None, "expected None");
    }

    #[test]
    fn read_reads_contents_of_file_as_string() {
        let got = read("tests/data/logbook.txt").unwrap().unwrap();
        assert_eq!(got.trim_end(), "hello world", "wrong text");
    }

    #[test]
    fn append_creates_file_if_necessary() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("newlog.txt");
        append(&path, "hello logbook").unwrap();
        let text = read_to_string(path).unwrap();
        assert_eq!(text, "hello logbook\n");
    }

    #[test]
    fn append_appends_to_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("newlog.txt");
        append(&path, "hello logbook").unwrap();
        append(&path, "hello world").unwrap();
        let text = read_to_string(path).unwrap();
        assert_eq!(text, "hello logbook\nhello world\n");
    }
}
