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
pub enum WriteBookTagsType {
    #[serde(rename = "newFiles")]
    NewFiles,
    #[serde(rename = "allFiles")]
    AllFiles,
    #[serde(rename = "sync")]
    Sync,

}

impl std::fmt::Display for WriteBookTagsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NewFiles => write!(f, "newFiles"),
            Self::AllFiles => write!(f, "allFiles"),
            Self::Sync => write!(f, "sync"),
        }
    }
}

impl Default for WriteBookTagsType {
    fn default() -> WriteBookTagsType {
        Self::NewFiles
    }
}

