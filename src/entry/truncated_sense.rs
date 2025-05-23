// 2 element array, with 0 = "sen", 1 = TruncatedSense

use super::{
    Etymology, GeneralLabel, Inflection, SenseSpecificGrammaticalLabel, SubjectStatusLabel,
    Variant, pronunciation::Pronunciation,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TruncatedSense {
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
