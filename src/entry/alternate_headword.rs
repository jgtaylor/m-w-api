use super::{pronunciation::Pronunciation, labels::ParenthesizedSubjectStatusLabel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternateHeadword {
    #[serde(rename = "hw")]
    pub word: String,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Vec<Pronunciation>>,
    #[serde(rename = "psl")]
    pub parenthesized_subject_status_label: Option<ParenthesizedSubjectStatusLabel>,
}