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
pub struct SeriesLazyLoaded {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<models::Series>>,
    #[serde(rename = "isLoaded", skip_serializing_if = "Option::is_none")]
    pub is_loaded: Option<bool>,
}

impl SeriesLazyLoaded {
    pub fn new() -> SeriesLazyLoaded {
        SeriesLazyLoaded {
            value: None,
            is_loaded: None,
        }
    }
}

