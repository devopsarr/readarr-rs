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
pub struct AuthorMetadata {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "foreignAuthorId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub foreign_author_id: Option<Option<String>>,
    #[serde(rename = "titleSlug", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title_slug: Option<Option<String>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "sortName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<Option<String>>,
    #[serde(rename = "nameLastFirst", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name_last_first: Option<Option<String>>,
    #[serde(rename = "sortNameLastFirst", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_name_last_first: Option<Option<String>>,
    #[serde(rename = "aliases", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Option<Vec<String>>>,
    #[serde(rename = "overview", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub overview: Option<Option<String>>,
    #[serde(rename = "disambiguation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub disambiguation: Option<Option<String>>,
    #[serde(rename = "gender", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Option<String>>,
    #[serde(rename = "hometown", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hometown: Option<Option<String>>,
    #[serde(rename = "born", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub born: Option<Option<String>>,
    #[serde(rename = "died", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub died: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::AuthorStatusType>,
    #[serde(rename = "images", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub images: Option<Option<Vec<models::MediaCover>>>,
    #[serde(rename = "links", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub links: Option<Option<Vec<models::Links>>>,
    #[serde(rename = "genres", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Option<Vec<String>>>,
    #[serde(rename = "ratings", skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Box<models::Ratings>>,
}

impl AuthorMetadata {
    pub fn new() -> AuthorMetadata {
        AuthorMetadata {
            id: None,
            foreign_author_id: None,
            title_slug: None,
            name: None,
            sort_name: None,
            name_last_first: None,
            sort_name_last_first: None,
            aliases: None,
            overview: None,
            disambiguation: None,
            gender: None,
            hometown: None,
            born: None,
            died: None,
            status: None,
            images: None,
            links: None,
            genres: None,
            ratings: None,
        }
    }
}

