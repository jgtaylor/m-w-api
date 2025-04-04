use crate::entry::quote::AttributionOfQuote;
use crate::tagged_vec;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VerbalIllustrationsKey {
    #[serde(rename = "vis")]
    Key,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerVerbalIllustration {
    #[serde(rename = "t")]
    pub text: String,
    #[serde(rename = "aq")]
    pub attribution_of_quote: Option<AttributionOfQuote>,
}

tagged_vec!(
    VerbalIllustration,
    VerbalIllustrationsKey,
    InnerVerbalIllustration
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verbal_illustration() {
        let data = r#"
        [
            "vis",
            [
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
        "#;
        let result: Result<VerbalIllustration, _> = serde_json::from_str(&data);
        match result {
            Ok(res) => println!("{:#?}", res),
            Err(err) => panic!("{}", err),
        }
    }
}
