use super::{
    inflection::Inflection,
    labels::{FunctionalLabel, GeneralLabel, ParenthesizedSubjectStatusLabel, SubjectStatusLabel},
    pronunciation::Pronunciation,
    usage::Usage,
    variant::Variant,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndefinedRunOn {
    #[serde(rename = "ure")]
    pub name: Option<String>,
    #[serde(rename = "fl")]
    pub functional_label: FunctionalLabel,
    #[serde(rename = "utxt")]
    pub text: Option<Vec<UndefinedRunOnText>>,
    #[serde(rename = "ins")]
    pub inflections: Option<Vec<Inflection>>,
    #[serde(rename = "lbs")]
    pub labels: Option<Vec<GeneralLabel>>,
    #[serde(rename = "prs")]
    pub pronunciations: Option<Vec<Pronunciation>>,
    #[serde(rename = "psl")]
    pub parenthesized_subect_status_label: Option<ParenthesizedSubjectStatusLabel>,
    #[serde(rename = "sls")]
    pub subject_status_labels: Option<Vec<SubjectStatusLabel>>,
    #[serde(rename = "rsl")]
    pub run_on_subject_status_label: Option<String>,
    #[serde(rename = "vrs")]
    pub variants: Option<Vec<Variant>>,
    #[serde(rename = "altprs", default)]
    pub alternate_pronounciation: Option<Vec<Pronunciation>>,
    #[serde(default)]
    pub gram: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UndefinedRunOnText {
    VerbalIllustrations(Vec<VerbalIllustration>),
    UsageNotes(Vec<Usage>),
    Wsgram(WithinSenseGram),
}
