use serde::{Deserialize, Serialize};
//use serde_json::value::Value;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Hna {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: String,
    pub title: String,
    pub url: String,
}

/// Builder for the `Hn` object.
#[derive(Default, Debug, Clone)]
pub struct HnaBuilder {
    by: Option<String>,
    title: Option<String>,
    url: Option<String>,
}

impl HnaBuilder {
    pub fn new<S: Into<String>>() -> HnaBuilder {
        HnaBuilder {
            ..Default::default()
        }
    }

    pub fn by<S: Into<String>>(mut self, by: S) -> Self {
        self.by = Some(by.into());
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn build(self) -> Result<Hna, String> {
        Ok(Hna {
            title: self.title.ok_or("title missing")?,
            by: self.by.ok_or("by missing")?,
            url: self.url.ok_or("url missing")?,
            ..Default::default()
        })
    }
}
