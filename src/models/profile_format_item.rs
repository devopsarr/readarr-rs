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
pub struct ProfileFormatItem {
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Box<models::CustomFormat>>,
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
}

impl ProfileFormatItem {
    pub fn new() -> ProfileFormatItem {
        ProfileFormatItem {
            format: None,
            score: None,
        }
    }
}

