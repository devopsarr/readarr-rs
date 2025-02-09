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
pub struct MediaInfoResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "audioChannels", skip_serializing_if = "Option::is_none")]
    pub audio_channels: Option<f64>,
    #[serde(rename = "audioBitRate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub audio_bit_rate: Option<Option<String>>,
    #[serde(rename = "audioCodec", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub audio_codec: Option<Option<String>>,
    #[serde(rename = "audioBits", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub audio_bits: Option<Option<String>>,
    #[serde(rename = "audioSampleRate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub audio_sample_rate: Option<Option<String>>,
}

impl MediaInfoResource {
    pub fn new() -> MediaInfoResource {
        MediaInfoResource {
            id: None,
            audio_channels: None,
            audio_bit_rate: None,
            audio_codec: None,
            audio_bits: None,
            audio_sample_rate: None,
        }
    }
}

