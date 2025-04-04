use super::{
    binding_substitute::BindingSubstitute, paren_sense_sequence::ParenSenseSequence, sense::Sense,
    truncated_sense::TruncatedSense,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Sequence {
    Sense(Sense),
    TruncatedSense(TruncatedSense),
    BindingSubstitute(BindingSubstitute),
    ParenSenseSequence(ParenSenseSequence),
}
