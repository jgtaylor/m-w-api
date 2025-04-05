use super::verbal_illustration::VerbalIllustration;
use crate::tagged_string;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DefinitionTextType {
    Text(DefiningText),
    VerbalIllustration(VerbalIllustration),
}

tagged_string!(DefiningText, "text");

#[cfg(test)]
mod tests {
    use serde_json::Error;

    use crate::entry::definition_text;

    use super::*;

    #[test]
    fn test_defining_text() {
        let myjson = r#"
            [
				"text",
				"{bc}of, relating to, or dealing with {d_link|aesthetics|aesthetic:2} or the beautiful "
			]
        "#;
        let result: Result<DefiningText, Error> = serde_json::from_str(&myjson);
        let _ = match result {
            Ok(res) => dbg!(res),
            Err(err) => panic!("{:#?}", err),
        };
    }

    #[test]
    fn test_defining_text_type() {
        let myjson = r#"
            [
					[
						"text",
						"{bc}of, relating to, or dealing with {d_link|aesthetics|aesthetic:2} or the beautiful "
					],
					[
						"vis",
		    				[
								{
									"t": "{wi}aesthetic{\/wi} theories"
								},
                                {
                                    "t": "admitted that there was much {wi}justice{/wi} in these observations",
                                    "aq": {
                                        "auth": "T. L. Peacock"
                                    }
                                },
                                {
                                    "t": "She was so tired she could hardly {it}get up{\/it} the energy to make dinner."
                                 }
							]
					]
			]
        "#;
        let result: Result<Vec<DefinitionTextType>, Error> = serde_json::from_str(&myjson);
        let _ = match result {
            Ok(res) => {
                for def_type in res {
                    let _ = match def_type {
                        DefinitionTextType::Text(t) => {
                            println!(
                                "DefiningText: {}",
                                <&definition_text::DefiningText as Into<&str>>::into(&t)
                            );
                        }
                        DefinitionTextType::VerbalIllustration(v) => {
                            for (idx, illustration_text) in v.iter().enumerate() {
                                println!("Illustration #{}: {}", idx, illustration_text.text);
                            }
                        }
                    };
                }
            }
            Err(err) => panic!("{:#?}", err),
        };
    }
}
