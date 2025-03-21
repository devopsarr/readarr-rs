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
pub struct Book {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "authorMetadataId", skip_serializing_if = "Option::is_none")]
    pub author_metadata_id: Option<i32>,
    #[serde(rename = "foreignBookId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub foreign_book_id: Option<Option<String>>,
    #[serde(rename = "foreignEditionId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub foreign_edition_id: Option<Option<String>>,
    #[serde(rename = "titleSlug", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title_slug: Option<Option<String>>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "releaseDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<Option<String>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Vec<models::Links>>>,
    #[serde(rename = "genres", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Option<Vec<String>>>,
    #[serde(rename = "relatedBooks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub related_books: Option<Option<Vec<i32>>>,
    #[serde(rename = "ratings", skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Box<models::Ratings>>,
    #[serde(rename = "lastSearchTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_search_time: Option<Option<String>>,
    #[serde(rename = "cleanTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub clean_title: Option<Option<String>>,
    #[serde(rename = "monitored", skip_serializing_if = "Option::is_none")]
    pub monitored: Option<bool>,
    #[serde(rename = "anyEditionOk", skip_serializing_if = "Option::is_none")]
    pub any_edition_ok: Option<bool>,
    #[serde(rename = "lastInfoSync", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_info_sync: Option<Option<String>>,
    #[serde(rename = "added", skip_serializing_if = "Option::is_none")]
    pub added: Option<String>,
    #[serde(rename = "addOptions", skip_serializing_if = "Option::is_none")]
    pub add_options: Option<Box<models::AddBookOptions>>,
    #[serde(rename = "authorMetadata", skip_serializing_if = "Option::is_none")]
    pub author_metadata: Option<Box<models::AuthorMetadataLazyLoaded>>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::AuthorLazyLoaded>>,
    #[serde(rename = "editions", skip_serializing_if = "Option::is_none")]
    pub editions: Option<Box<models::EditionListLazyLoaded>>,
    #[serde(rename = "bookFiles", skip_serializing_if = "Option::is_none")]
    pub book_files: Option<Box<models::BookFileListLazyLoaded>>,
    #[serde(rename = "seriesLinks", skip_serializing_if = "Option::is_none")]
    pub series_links: Option<Box<models::SeriesBookLinkListLazyLoaded>>,
}

impl Book {
    pub fn new() -> Book {
        Book {
            id: None,
            author_metadata_id: None,
            foreign_book_id: None,
            foreign_edition_id: None,
            title_slug: None,
            title: None,
            release_date: None,
            links: None,
            genres: None,
            related_books: None,
            ratings: None,
            last_search_time: None,
            clean_title: None,
            monitored: None,
            any_edition_ok: None,
            last_info_sync: None,
            added: None,
            add_options: None,
            author_metadata: None,
            author: None,
            editions: None,
            book_files: None,
            series_links: None,
        }
    }
}

