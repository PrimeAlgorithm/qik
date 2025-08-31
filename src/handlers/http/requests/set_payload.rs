//! Handles setting and keeping track of payload related information.

use crate::commands::http::PayloadArgs;
use bytes::Bytes;
use reqwest::multipart::{Form, Part};
use std::collections::HashMap;

/// HTTP request body content with its associated MIME type.
pub struct BodyInfo {
    pub content: Bytes,

    pub content_type: Option<String>,
}

/// Represents different types of HTTP request payloads that can be sent or received.
pub enum Payload {
    Body(BodyInfo),
    Multipart(Form),
    Form(HashMap<String, String>),
    None,
}

/// Converts payload arguments into HTTP [`Payload`] format.
pub async fn set_payload(payload_args: &PayloadArgs) -> Result<Payload, anyhow::Error> {
    if let Some(raw) = &payload_args.raw {
        return Ok(Payload::Body(BodyInfo {
            content: Bytes::from(raw.clone()),
            content_type: None,
        }));
    }

    if let Some(json) = &payload_args.json {
        return Ok(Payload::Body(BodyInfo {
            content: Bytes::from(json.clone()),
            content_type: Some("application/json".to_string()),
        }));
    }

    if let Some(form_data_list) = &payload_args.form {
        let mut url_params = HashMap::new();
        let mut form = Form::new();
        let mut multipart = false;

        for form_data in form_data_list.iter() {
            if let Some(file_path) = &form_data.file_path {
                multipart = true;
                let mut part = Part::file(file_path.clone()).await?;

                if let Some(file_name) = &form_data.file_name {
                    part = part.file_name(file_name.clone());
                }
                form = form.part(form_data.key.clone(), part);
            } else if let Some(str) = &form_data.str_value {
                url_params.insert(form_data.key.clone(), str.clone());
            }
        }

        if multipart {
            for (key, value) in url_params.into_iter() {
                form = form.text(key, value);
            }

            return Ok(Payload::Multipart(form));
        } else {
            return Ok(Payload::Form(url_params));
        }
    }

    if let Some(xml) = &payload_args.xml {
        return Ok(Payload::Body(BodyInfo {
            content: Bytes::from(xml.clone()),
            content_type: Some("application/xml".to_string()),
        }));
    }

    Ok(Payload::None)
}
