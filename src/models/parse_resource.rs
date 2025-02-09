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
pub struct ParseResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "parsedBookInfo", skip_serializing_if = "Option::is_none")]
    pub parsed_book_info: Option<Box<models::ParsedBookInfo>>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::AuthorResource>>,
    #[serde(rename = "books", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub books: Option<Option<Vec<models::BookResource>>>,
}

impl ParseResource {
    pub fn new() -> ParseResource {
        ParseResource {
            id: None,
            title: None,
            parsed_book_info: None,
            author: None,
            books: None,
        }
    }
}

