#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadwordInformation {
    #[serde(rename = "hw")]
    pub word: String,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Pronunciations>, // prob vec
    #[serde(rename = "altprs", default)]
    pub alternate_pronounciation: Option<AlternatePronounciation>,
}
