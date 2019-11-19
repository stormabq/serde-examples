use serde::{Deserialize, Serialize};
//use serde_json::value::Value;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Hna {
    pub by: String,
    pub descendants: i32,
    pub id: i32,
    pub kids: Vec<i32>,
    pub score: i32,
    pub time: i32,
    pub title: String,
    pub r#type: String,
    pub url: String,
}

/// Builder for the `Hn` object.
#[derive(Default, Debug, Clone)]
pub struct HnaBuilder {
    by: Option<String>,
    descendants: Option<i32>,
    id: Option<i32>,
    kids: Option<Vec<i32>>,
    score: Option<i32>,
    time: Option<i32>,
    title: Option<String>,
    r#type: Option<String>,
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

    pub fn descendants<S: Into<i32>>(mut self, descendents: S) -> Self {
        self.id = Some(descendents.into());
        self
    }

    pub fn id<S: Into<i32>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn kids<S: Into<Vec<i32>>>(mut self, kids: S) -> Self {
        self.kids = Some(kids.into());
        self
    }

    pub fn score<S: Into<i32>>(mut self, score: S) -> Self {
        self.score = Some(score.into());
        self
    }

    pub fn time<S: Into<i32>>(mut self, time: S) -> Self {
        self.time = Some(time.into());
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn r#type<S: Into<String>>(mut self, r#type: S) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn build(self) -> Result<Hna, String> {
        Ok(Hna {
            by: self.by.ok_or("by missing")?,
            descendants: self.descendants.ok_or("by missing")?,
            id: self.id.ok_or("id missing")?,
            kids: self.kids.ok_or("kids missing")?,
            score: self.score.ok_or("score missing")?,
            time: self.time.ok_or("time missing")?,
            title: self.title.ok_or("title missing")?,
            r#type: self.r#type.ok_or("type missing")?,
            url: self.url.ok_or("url missing")?,
            ..Default::default()
        })
    }
}
