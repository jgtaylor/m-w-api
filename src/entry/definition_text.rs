use super::verbal_illustration::VerbalIllustration;
use crate::tagged_string;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DefinitionTextType {
    Text(DefiningText),
    VerbalIllustrations(Vec<VerbalIllustration>),

}

tagged_string!(DefiningText, "text");

