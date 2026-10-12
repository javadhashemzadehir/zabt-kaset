use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DuplicateMatch {
    Sha256,
    VideoId,
    Metadata,
    Filename,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateHit {
    pub matched_by: DuplicateMatch,
    pub key: String,
}

pub fn sha256_file(path: &Path) -> AppResult<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

pub fn match_video_id(existing: &str, candidate: &str) -> Option<DuplicateHit> {
    if !existing.is_empty() && existing == candidate {
        Some(DuplicateHit {
            matched_by: DuplicateMatch::VideoId,
            key: existing.to_string(),
        })
    } else {
        None
    }
}

pub fn match_filename(existing: &str, candidate: &str) -> Option<DuplicateHit> {
    if existing.eq_ignore_ascii_case(candidate) {
        Some(DuplicateHit {
            matched_by: DuplicateMatch::Filename,
            key: existing.to_string(),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn hashes_file() {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "zabt-kaset").unwrap();
        let hash = sha256_file(f.path()).unwrap();
        assert_eq!(hash.len(), 64);
    }
}
