/*
 * Readarr
 *
 * Readarr API docs
 *
 * The version of the OpenAPI document: v0.4.10.2734
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`create_history_failed_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateHistoryFailedByIdError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHistoryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_history_author`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHistoryAuthorError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_history_since`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHistorySinceError {
    UnknownValue(serde_json::Value),
}


pub async fn create_history_failed_by_id(configuration: &configuration::Configuration, id: i32) -> Result<(), Error<CreateHistoryFailedByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!("{}/api/v1/history/failed/{id}", configuration.base_path, id=p_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateHistoryFailedByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_history(configuration: &configuration::Configuration, page: Option<i32>, page_size: Option<i32>, sort_key: Option<&str>, sort_direction: Option<models::SortDirection>, include_author: Option<bool>, include_book: Option<bool>, event_type: Option<Vec<i32>>, book_id: Option<i32>, download_id: Option<&str>) -> Result<models::HistoryResourcePagingResource, Error<GetHistoryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_page = page;
    let p_page_size = page_size;
    let p_sort_key = sort_key;
    let p_sort_direction = sort_direction;
    let p_include_author = include_author;
    let p_include_book = include_book;
    let p_event_type = event_type;
    let p_book_id = book_id;
    let p_download_id = download_id;

    let uri_str = format!("{}/api/v1/history", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("pageSize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort_key {
        req_builder = req_builder.query(&[("sortKey", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort_direction {
        req_builder = req_builder.query(&[("sortDirection", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_author {
        req_builder = req_builder.query(&[("includeAuthor", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_book {
        req_builder = req_builder.query(&[("includeBook", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_event_type {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("eventType".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("eventType", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref param_value) = p_book_id {
        req_builder = req_builder.query(&[("bookId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_download_id {
        req_builder = req_builder.query(&[("downloadId", &param_value.to_string())]);
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<GetHistoryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_history_author(configuration: &configuration::Configuration, author_id: Option<i32>, book_id: Option<i32>, event_type: Option<models::EntityHistoryEventType>, include_author: Option<bool>, include_book: Option<bool>) -> Result<Vec<models::HistoryResource>, Error<ListHistoryAuthorError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_author_id = author_id;
    let p_book_id = book_id;
    let p_event_type = event_type;
    let p_include_author = include_author;
    let p_include_book = include_book;

    let uri_str = format!("{}/api/v1/history/author", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_author_id {
        req_builder = req_builder.query(&[("authorId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_book_id {
        req_builder = req_builder.query(&[("bookId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_event_type {
        req_builder = req_builder.query(&[("eventType", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_author {
        req_builder = req_builder.query(&[("includeAuthor", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_book {
        req_builder = req_builder.query(&[("includeBook", &param_value.to_string())]);
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ListHistoryAuthorError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_history_since(configuration: &configuration::Configuration, date: Option<String>, event_type: Option<models::EntityHistoryEventType>, include_author: Option<bool>, include_book: Option<bool>) -> Result<Vec<models::HistoryResource>, Error<ListHistorySinceError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_date = date;
    let p_event_type = event_type;
    let p_include_author = include_author;
    let p_include_book = include_book;

    let uri_str = format!("{}/api/v1/history/since", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_date {
        req_builder = req_builder.query(&[("date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_event_type {
        req_builder = req_builder.query(&[("eventType", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_author {
        req_builder = req_builder.query(&[("includeAuthor", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_book {
        req_builder = req_builder.query(&[("includeBook", &param_value.to_string())]);
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<ListHistorySinceError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

