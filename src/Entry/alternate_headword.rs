#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternateHeadword {
    #[serde(rename = "hw")]
    pub word: String,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Pronunciations>,
    #[serde(rename = "psl")]
    pub parenthesized_subject_status_label: Option<ParenthesizedSubjectStatusLabel>,
}