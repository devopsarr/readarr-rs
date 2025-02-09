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
pub struct Revision {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "real", skip_serializing_if = "Option::is_none")]
    pub real: Option<i32>,
    #[serde(rename = "isRepack", skip_serializing_if = "Option::is_none")]
    pub is_repack: Option<bool>,
}

impl Revision {
    pub fn new() -> Revision {
        Revision {
            version: None,
            real: None,
            is_repack: None,
        }
    }
}

