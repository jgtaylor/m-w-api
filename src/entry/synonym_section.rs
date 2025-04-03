use super::verbal_illustration::VerbalIllustration;
use crate::tagged_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynonymDiscussion {
    #[serde(rename = "pl")]
    pub label: String,
    #[serde(rename = "pt")]
    pub text: Vec<SynonymTextType>,
    #[serde(rename = "sarefs")]
    pub see_also_references: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SynonymTextType {
    Text(SynonymText),
    VerbalIllustrations(Vec<VerbalIllustration>),
}

tagged_string!(SynonymText, "text");
