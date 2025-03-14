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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexerConfigResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "minimumAge", skip_serializing_if = "Option::is_none")]
    pub minimum_age: Option<i32>,
    #[serde(rename = "maximumSize", skip_serializing_if = "Option::is_none")]
    pub maximum_size: Option<i32>,
    #[serde(rename = "retention", skip_serializing_if = "Option::is_none")]
    pub retention: Option<i32>,
    #[serde(rename = "rssSyncInterval", skip_serializing_if = "Option::is_none")]
    pub rss_sync_interval: Option<i32>,
}

impl IndexerConfigResource {
    pub fn new() -> IndexerConfigResource {
        IndexerConfigResource {
            id: None,
            minimum_age: None,
            maximum_size: None,
            retention: None,
            rss_sync_interval: None,
        }
    }
}

