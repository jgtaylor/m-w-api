#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    #[serde(rename = "pl")]
    pub label: String,
    #[serde(rename = "pt")]
    pub text: Vec<UsageText>,
    #[serde(rename = "uarefs")]
    pub see_in_addition: Option<Vec<UsageInAddition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsageText {
    Text(TextUsageType),
    VerbalIllustrations(Vec<VerbalIllustration>),
}

tagged_string!(TextUsageType, "text");


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInAddition {
    pub uarefs: String,
}
