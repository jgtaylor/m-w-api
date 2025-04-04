use super::{
    Etymology, GeneralLabel, Inflection, SenseSpecificGrammaticalLabel, SubjectStatusLabel,
    Variant, definition_text::DefinitionTextType, pronunciation::Pronunciation,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DividedSense {
    #[serde(rename = "sd")]
    pub divider: String,
    #[serde(rename = "dt")]
    pub definition_text: Vec<DefinitionTextType>,

    #[serde(rename = "et", default)]
    etymology: Option<Vec<Etymology>>,
    #[serde(rename = "ins", default)]
    inflections: Option<Vec<Inflection>>,
    #[serde(rename = "lbs", default)]
    labels: Option<Vec<GeneralLabel>>,
    #[serde(rename = "prs", default)]
    pronunciations: Option<Vec<Pronunciation>>,
    #[serde(rename = "sgram", default)]
    sense_specific_grammtical_label: Option<SenseSpecificGrammaticalLabel>,
    #[serde(rename = "sls", default)]
    subject_status_label: Option<SubjectStatusLabel>,
    #[serde(rename = "sn")]
    sense_number: Option<String>,
    #[serde(rename = "vrs", default)]
    variants: Option<Vec<Variant>>,
}
