#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionSection {
    #[serde(rename = "vd")]
    pub verb_divider: Option<VerbDivider>,
    #[serde(rename = "sseq")]
    pub sense_sequence: Vec<SenseSequence>,
}
