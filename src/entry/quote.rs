#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    #[serde(rename = "t")]
    pub text: String,
    #[serde(rename = "aq")]
    pub attribution_of_quote: Option<AttributionOfQuote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionOfQuote {
    #[serde(rename = "auth")]
    pub author: String,
    pub source: Option<String>,
    #[serde(rename = "aqdate")]
    pub publication_date: Option<String>,
    pub subsource: Option<SubSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubSource {
    pub source: String,
    #[serde(rename = "aqdate")]
    pub publication_date: String,
}