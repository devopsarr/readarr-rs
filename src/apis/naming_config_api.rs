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
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`get_naming_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNamingConfigError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_naming_config_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNamingConfigByIdError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_naming_config_examples`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNamingConfigExamplesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_naming_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNamingConfigError {
    UnknownValue(serde_json::Value),
}


pub async fn get_naming_config(configuration: &configuration::Configuration, ) -> Result<models::NamingConfigResource, Error<GetNamingConfigError>> {

    let uri_str = format!("{}/api/v1/config/naming", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::NamingConfigResource`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::NamingConfigResource`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetNamingConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_naming_config_by_id(configuration: &configuration::Configuration, id: i32) -> Result<models::NamingConfigResource, Error<GetNamingConfigByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!("{}/api/v1/config/naming/{id}", configuration.base_path, id=p_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::NamingConfigResource`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::NamingConfigResource`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetNamingConfigByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_naming_config_examples(configuration: &configuration::Configuration, rename_books: Option<bool>, replace_illegal_characters: Option<bool>, colon_replacement_format: Option<i32>, standard_book_format: Option<&str>, author_folder_format: Option<&str>, include_author_name: Option<bool>, include_book_title: Option<bool>, include_quality: Option<bool>, replace_spaces: Option<bool>, separator: Option<&str>, number_style: Option<&str>, id: Option<i32>, resource_name: Option<&str>) -> Result<(), Error<GetNamingConfigExamplesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_rename_books = rename_books;
    let p_replace_illegal_characters = replace_illegal_characters;
    let p_colon_replacement_format = colon_replacement_format;
    let p_standard_book_format = standard_book_format;
    let p_author_folder_format = author_folder_format;
    let p_include_author_name = include_author_name;
    let p_include_book_title = include_book_title;
    let p_include_quality = include_quality;
    let p_replace_spaces = replace_spaces;
    let p_separator = separator;
    let p_number_style = number_style;
    let p_id = id;
    let p_resource_name = resource_name;

    let uri_str = format!("{}/api/v1/config/naming/examples", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_rename_books {
        req_builder = req_builder.query(&[("renameBooks", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_replace_illegal_characters {
        req_builder = req_builder.query(&[("replaceIllegalCharacters", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_colon_replacement_format {
        req_builder = req_builder.query(&[("colonReplacementFormat", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_standard_book_format {
        req_builder = req_builder.query(&[("standardBookFormat", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_author_folder_format {
        req_builder = req_builder.query(&[("authorFolderFormat", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_author_name {
        req_builder = req_builder.query(&[("includeAuthorName", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_book_title {
        req_builder = req_builder.query(&[("includeBookTitle", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_include_quality {
        req_builder = req_builder.query(&[("includeQuality", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_replace_spaces {
        req_builder = req_builder.query(&[("replaceSpaces", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_separator {
        req_builder = req_builder.query(&[("separator", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_number_style {
        req_builder = req_builder.query(&[("numberStyle", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_id {
        req_builder = req_builder.query(&[("id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_resource_name {
        req_builder = req_builder.query(&[("resourceName", &param_value.to_string())]);
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
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<GetNamingConfigExamplesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_naming_config(configuration: &configuration::Configuration, id: &str, naming_config_resource: Option<models::NamingConfigResource>) -> Result<models::NamingConfigResource, Error<UpdateNamingConfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_naming_config_resource = naming_config_resource;

    let uri_str = format!("{}/api/v1/config/naming/{id}", configuration.base_path, id=crate::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

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
    req_builder = req_builder.json(&p_naming_config_resource);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::NamingConfigResource`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::NamingConfigResource`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateNamingConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

