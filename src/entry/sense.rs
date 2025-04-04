use super::{pronunciation::Pronunciation, Etymology, GeneralLabel, Inflection, SubjectStatusLabel, SenseSpecificGrammaticalLabel, Variant};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sense {
    #[serde(rename = "sn")]
    sense_number: Option<String>,
    #[serde(rename = "dt")]
    definition_texts: Vec<DefinitionTextType>,
    #[serde(rename = "et", default)]
    etymology: Option<Vec<Etymology>>,
    #[serde(rename = "ins", default)]
    inflections: Option<Vec<Inflection>>,
    #[serde(rename = "lbs", default)]
    labels: Option<Vec<GeneralLabel>>,
    #[serde(rename = "prs", default)]
    pronunciations: Option<Vec<Pronunciation>>,
    #[serde(rename = "sdsense", default)]
    divided_sense: Option<DividedSense>,
    #[serde(rename = "sgram", default)]
    sense_specific_grammtical_label: Option<SenseSpecificGrammaticalLabel>,
    #[serde(rename = "sls", default)]
    subject_status_label: Option<SubjectStatusLabel>,
    #[serde(rename = "vrs", default)]
    variants: Option<Vec<Variant>>,
}