use crate::api_error::ApiError;
use serde::Serialize;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref SENDINBLUE_API_KEY: String = std::env::var("SENDINBLUE_API_KEY").unwrap_or("".to_string());
}

#[derive(Debug, Serialize)]
pub struct Contact {
    email: String,
    name: Option<String>,
}

impl Contact {
    pub fn new<T: Into<String>>(email: T, name: T) -> Self {
        Contact { email: email.into(), name: Some(name.into()) }
    }
}

impl<T: Into<String>> From<T> for Contact {
    fn from(email: T) -> Self {
        Contact { email: email.into(), name: None }
    }
}

#[derive(Debug, Serialize)]
pub struct Email {
    sender: Contact,
    #[serde(rename = "to")]
    recipients: Vec<Contact>,
    subject: String,
    #[serde(rename = "htmlContent")]
    html: Option<String>
}

impl Email {
    pub fn new(sender: Contact) -> Self {
        Email {
            sender,
            recipients: Vec::new(),
            subject: "".to_string(),
            html: None,
        }
    }

    pub fn add_recipient<T: Into<Contact>>(mut self, recipient: T) -> Self {
        self.recipients.push(recipient.into());
        self
    }

    pub fn set_subject<T: Into<String>>(mut self, subject: T) -> Self {
        self.subject = subject.into();
        self
    }

    pub fn set_html<T: Into<String>>(mut self, html: T) -> Self {
        self.html = Some(html.into());
        self
    }

    pub fn send(self) -> Result<String, ApiError> {
        let client = reqwest::Client::new();
        let mut response = client.post("https://api.sendinblue.com/v3/smtp/email")
            .header("Accept", "application/json")
            .header("api-key", SENDINBLUE_API_KEY.as_str())
            .json(&self)
            .send()
            .map_err(|e| ApiError::new(500, format!("Failed to send email: {}", e)))?;

        let status = response.status().as_u16();
        let mut body: HashMap<String, String> = response
            .json()
            .map_err(|e| ApiError::new(500, format!("Failed to read sendinblue response: {}", e)))?;

        match status {
            201 => Ok(body.remove("messageId").unwrap_or("".to_string())),
            _ => {
                let message = body.remove("message").unwrap_or("Unknown error".to_string());
                Err(ApiError::new(500, format!("Failed to send email: {}", message)))
            }
        }
    }
}
