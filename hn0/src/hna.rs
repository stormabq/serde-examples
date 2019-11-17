use serde::{Deserialize, Serialize};
use serde_json::value::Value;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Hna {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub address: Option<String>,
}

/// Builder for the `Hn` object.
#[derive(Default, Debug, Clone)]
pub struct StatusBuilder {
    space: String,
    logo: Option<String>,
    url: Option<String>,
}

impl StatusBuilder {
    pub fn new<S: Into<String>>(space_name: S) -> StatusBuilder {
        StatusBuilder {
            space: space_name.into(),
            ..Default::default()
        }
    }

    pub fn logo<S: Into<String>>(mut self, logo: S) -> Self {
        self.logo = Some(logo.into());
        self
    }

    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn build(self) -> Result<Hna, String> {
        Ok(Hna {
            api: "0.13".into(), // TODO: Deduplicate
            space: self.space,
            logo: self.logo.ok_or("logo missing")?,
            url: self.url.ok_or("url missing")?,
            ..Default::default()
        })
    }
}
