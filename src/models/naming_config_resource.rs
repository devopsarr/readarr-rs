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
pub struct NamingConfigResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "renameBooks", skip_serializing_if = "Option::is_none")]
    pub rename_books: Option<bool>,
    #[serde(rename = "replaceIllegalCharacters", skip_serializing_if = "Option::is_none")]
    pub replace_illegal_characters: Option<bool>,
    #[serde(rename = "colonReplacementFormat", skip_serializing_if = "Option::is_none")]
    pub colon_replacement_format: Option<i32>,
    #[serde(rename = "standardBookFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub standard_book_format: Option<Option<String>>,
    #[serde(rename = "authorFolderFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub author_folder_format: Option<Option<String>>,
    #[serde(rename = "includeAuthorName", skip_serializing_if = "Option::is_none")]
    pub include_author_name: Option<bool>,
    #[serde(rename = "includeBookTitle", skip_serializing_if = "Option::is_none")]
    pub include_book_title: Option<bool>,
    #[serde(rename = "includeQuality", skip_serializing_if = "Option::is_none")]
    pub include_quality: Option<bool>,
    #[serde(rename = "replaceSpaces", skip_serializing_if = "Option::is_none")]
    pub replace_spaces: Option<bool>,
    #[serde(rename = "separator", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub separator: Option<Option<String>>,
    #[serde(rename = "numberStyle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub number_style: Option<Option<String>>,
}

impl NamingConfigResource {
    pub fn new() -> NamingConfigResource {
        NamingConfigResource {
            id: None,
            rename_books: None,
            replace_illegal_characters: None,
            colon_replacement_format: None,
            standard_book_format: None,
            author_folder_format: None,
            include_author_name: None,
            include_book_title: None,
            include_quality: None,
            replace_spaces: None,
            separator: None,
            number_style: None,
        }
    }
}

