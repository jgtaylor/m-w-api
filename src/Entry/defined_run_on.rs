
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinedRunOn {
    #[serde(rename = "drp")]
    pub value: Option<String>,
    #[serde(rename = "def")]
    pub definitions: Vec<DefinitionSection>,
    #[serde(rename = "utxt")]
    pub text: Option<Vec<DefinedRunOnText>>,
    #[serde(rename = "ins")]
    pub inflections: Option<Inflections>,
    #[serde(rename = "lbs")]
    pub labels: Option<GeneralLabels>,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Pronunciations>,
    #[serde(rename = "psl")]
    pub parenthesized_subect_status_label: Option<ParenthesizedSubjectStatusLabel>,
    #[serde(rename = "rsl")]
    pub run_on_subject_status_label: Option<String>,
    #[serde(rename = "sls")]
    pub subject_status_labels: Option<SubjectStatusLabels>,
    #[serde(rename = "vrs")]
    pub variants: Option<Vec<Variant>>,
    #[serde(default)]
    pub gram: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinedRunOnText {
    VerbalIllustrations(Vec<VerbalIllustration>),
    VerbalIllustration(VerbalIllustration),
    UsageNotes(Vec<Usage>),
    UsageNote(Usage),
}
