#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    #[serde(rename = "va")]
    pub name: String,
    #[serde(rename = "vl")]
    pub label: Option<String>,
    #[serde(rename = "prs")]
    pub pronunciation: Option<Pronunciations>,
    #[serde(rename = "spl")]
    pub spl: Option<SenseSpecificInflectionPluralLabel>,
    #[serde(rename = "altprs", default)]
    pub alternate_pronounciation: Option<AlternatePronounciation>,
}
