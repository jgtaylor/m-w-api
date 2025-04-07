use super::{
    Etymology, GeneralLabel, Inflection, SenseSpecificGrammaticalLabel, SubjectStatusLabel,
    Variant, definition_text::DefinitionTextType, divided_sense::DividedSense,
    pronunciation::Pronunciation,
};
use crate::tagged_object;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SenseKey {
    #[serde(rename = "sense")]
    Key,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
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

tagged_object!(SenseObject, SenseKey, Sense);

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::Error;

    #[test]
    fn test_sense_object() {
        let myjson = r#"
        [
			"sense",
			{
				"sn": "1 a",
				"dt": [
					[
						"text",
						"{bc}of, relating to, or dealing with {d_link|aesthetics|aesthetic:2} or the beautiful "
					],
					[
						"vis",
		    				[
								{
									"t": "{wi}aesthetic{\/wi} theories"
								}
							]
					]
				]
			}
		]
        "#;

        let result: Result<Vec<SenseObject>, Error> = serde_json::from_str(&myjson);
        let _ = match result {
            Ok(res) => {
                dbg!(res)
            }
            Err(err) => panic!("{}", err),
        };
    }
}
