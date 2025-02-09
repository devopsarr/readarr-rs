/*
 * Readarr
 *
 * Readarr API docs
 *
 * The version of the OpenAPI document: v0.4.10.2734
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DownloadProtocol {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "usenet")]
    Usenet,
    #[serde(rename = "torrent")]
    Torrent,

}

impl std::fmt::Display for DownloadProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Usenet => write!(f, "usenet"),
            Self::Torrent => write!(f, "torrent"),
        }
    }
}

impl Default for DownloadProtocol {
    fn default() -> DownloadProtocol {
        Self::Unknown
    }
}

