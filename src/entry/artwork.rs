#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "art")]
pub struct Artwork {
    #[serde(rename = "artid")]
    pub id: String,
    #[serde(rename = "capt")]
    pub caption: Option<String>,
    pub dimensions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "artl")]
pub struct ArtworkLearners {
    pub art: Artwork,
}

impl Artwork {
    pub fn to_url(&self) -> Option<String> {
        if let Some(point_in_string) = self.id.rfind('.') {
            let x = &self.id[..point_in_string];
            Some(format!("http://www.learnersdictionary.com/art/ld/{}.gif", &x))
        } else {
            None
        }
    }
}
