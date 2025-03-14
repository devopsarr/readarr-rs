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
pub struct AuthorTitleInfo {
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "titleWithoutYear", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title_without_year: Option<Option<String>>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl AuthorTitleInfo {
    pub fn new() -> AuthorTitleInfo {
        AuthorTitleInfo {
            title: None,
            title_without_year: None,
            year: None,
        }
    }
}

