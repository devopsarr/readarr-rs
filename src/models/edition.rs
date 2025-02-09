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
pub struct Edition {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "bookId", skip_serializing_if = "Option::is_none")]
    pub book_id: Option<i32>,
    #[serde(rename = "foreignEditionId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub foreign_edition_id: Option<Option<String>>,
    #[serde(rename = "titleSlug", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title_slug: Option<Option<String>>,
    #[serde(rename = "isbn13", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub isbn13: Option<Option<String>>,
    #[serde(rename = "asin", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub asin: Option<Option<String>>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "language", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub language: Option<Option<String>>,
    #[serde(rename = "overview", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub overview: Option<Option<String>>,
    #[serde(rename = "format", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub format: Option<Option<String>>,
    #[serde(rename = "isEbook", skip_serializing_if = "Option::is_none")]
    pub is_ebook: Option<bool>,
    #[serde(rename = "disambiguation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub disambiguation: Option<Option<String>>,
    #[serde(rename = "publisher", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Option<String>>,
    #[serde(rename = "pageCount", skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i32>,
    #[serde(rename = "releaseDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<Option<String>>,
    #[serde(rename = "images", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub images: Option<Option<Vec<models::MediaCover>>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Vec<models::Links>>>,
    #[serde(rename = "ratings", skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Box<models::Ratings>>,
    #[serde(rename = "monitored", skip_serializing_if = "Option::is_none")]
    pub monitored: Option<bool>,
    #[serde(rename = "manualAdd", skip_serializing_if = "Option::is_none")]
    pub manual_add: Option<bool>,
    #[serde(rename = "book", skip_serializing_if = "Option::is_none")]
    pub book: Option<Box<models::BookLazyLoaded>>,
    #[serde(rename = "bookFiles", skip_serializing_if = "Option::is_none")]
    pub book_files: Option<Box<models::BookFileListLazyLoaded>>,
}

impl Edition {
    pub fn new() -> Edition {
        Edition {
            id: None,
            book_id: None,
            foreign_edition_id: None,
            title_slug: None,
            isbn13: None,
            asin: None,
            title: None,
            language: None,
            overview: None,
            format: None,
            is_ebook: None,
            disambiguation: None,
            publisher: None,
            page_count: None,
            release_date: None,
            images: None,
            links: None,
            ratings: None,
            monitored: None,
            manual_add: None,
            book: None,
            book_files: None,
        }
    }
}

