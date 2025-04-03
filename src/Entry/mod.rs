use serde::{Deserialize, Serialize};

pub mod entry_metadata;
use entry_metadata::EntryMetadata;
pub mod headword_information;
use headword_information::HeadwordInformation;
pub mod homograph;
use homograph::Homograph;
pub mod alternate_headword;
use alternate_headword::AlternateHeadword;
pub mod variant;
pub use variant::Variant;
pub mod labels;
pub use labels::*;
pub mod inflection;
pub use inflection::Inflection;
pub mod cognate_cross_reference;
pub use cognate_cross_reference::CognateCrossReference;
pub mod definition_section;
pub use definition_section::DefinitionSection;
pub mod undefined_run_on;
pub use undefined_run_on::UndefinedRunOn;
pub mod defined_run_on;
pub use defined_run_on::DefinedRunOn;
pub mod etymology;
pub use etymology::Etymology;
pub mod usage;
pub use usage::Usage;
pub mod synonym_section;
pub use synonym_section::SynonymDiscussion;
pub mod quote;
pub use quote::Quote;
pub mod artwork;
pub use artwork::{Artwork, ArtworkLearners};
pub mod table;
pub use table::Table;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "meta")]
    pub entry_metadata: EntryMetadata,
    #[serde(rename = "hwi")]
    pub headword_information: HeadwordInformation,
    #[serde(rename = "hom", default)]
    pub homograph: Option<Homograph>,
    #[serde(default, rename = "ahws")]
    pub alternate_headwords: Option<Vec<AlternateHeadword>>,
    #[serde(default, rename = "vrs")]
    pub variants: Option<Vec<Variant>>,
    #[serde(default, rename = "fl")]
    pub function_label: Option<FunctionalLabel>,
    #[serde(default, rename = "lbs")]
    pub general_labels: Option<Vec<GeneralLabel>>,
    #[serde(default, rename = "ins")]
    pub inflections: Option<Vec<Inflection>>,
    #[serde(default, rename = "cxs")]
    pub cognate_cross_references: Option<Vec<CognateCrossReference>>,
    #[serde(default, rename = "def")]
    pub definition_sections: Option<Vec<DefinitionSection>>, 
    #[serde(default, rename = "uros")]
    pub undefined_run_ons: Option<Vec<UndefinedRunOn>>, 
    #[serde(default, rename = "dros")]
    pub defined_run_ons: Option<Vec<DefinedRunOn>>, 
    #[serde(default, rename = "et")]
    pub etymology: Option<Vec<Etymology>>,
    #[serde(default, rename = "dxnls")]
    pub directional_cross_reference_section: Option<Vec<String>>,

    /// # 2.21 Usage Section: usages
    /// 
    /// In addition to usage notes within definitions, a more extensive usage discussion may be included in the entry. A set of usage discussions makes up a usage section, which is contained in `usages`.
    /// 
    /// ## Hierarchical Context
    /// 
    /// Top-level member of dictionary entry, occurring near end of entry.
    /// 
    /// ## Display Guidance
    /// 
    /// Typically displayed in a separate paragraph, using the pl contents as a heading.
    ///
    /// A `uarefs` is preceded by "usage " in bold, then "see in addition " in normal font, followed by one or more uaref members rendered as hyperlinks to other usage sections.
    ///
    /// ## Data Model
    ///
    /// `"usages"` : array of one or more usage objects containing the members:
    /// 
    /// `"pl"` : string  paragraph label: heading to display at top of section
    /// 
    /// `"pt"` : \[array\] paragraph text containing the elements:
    /// 
    /// `"text"`, string  usage discussion text
    /// 
    /// `vis`
    /// 
    /// `"uarefs"` : object  containing one or more:
    /// 
    /// `"uaref"` : string  usage see in addition reference: contains the text and ID of a "see in addition" reference to another usage section.
    ///  
    ///  [API Documentation ↗︎](https://dictionaryapi.com/products/json#sec-2.usages)
    #[serde(default, rename = "usages")]
    pub usage_section: Option<Vec<Usage>>,

    /// # 2.22 Synonyms Section: `syns`
    /// https://dictionaryapi.com/products/json#sec-2.syns
    /// 
    /// Extensive discussions of synonyms for the headword may be included in the entry. A set of such synonym discussions makes up a synonym section, which is contained in syns.
    /// 
    /// ## Hierarchical Context
    /// 
    /// Top-level member of dictionary entry, occurring near end of entry.
    /// 
    /// ## Display Guidance
    /// 
    /// Typically displayed in a separate paragraph with a heading such as "Synonym Discussion of [headword]".
    /// 
    /// An sarefs is preceded by "synonyms " in bold, then "see in addition " in normal font, then the sarefs array elements rendered as hyperlinks to other synonym sections.
    /// 
    /// ## Data Model
    /// `"syns"` : array of one or more synonym discussion objects containing the members:
    /// 
    /// `"pl"` : string  paragraph label: heading to display at top of section
    /// 
    /// `"pt"` : [array]  paragraph text containing the elements:
    /// 
    /// `"text"`, string  synonym discussion text
    /// 
    /// `vis`
    /// 
    /// `"sarefs"` : array  see in addition reference: contains one or more elements, each of which is the text and ID of a "see in addition" reference to another synonym section.
    #[serde(default, rename = "syns")]
    pub synonym_section: Option<SynonymDiscussion>,

    #[serde(default, rename = "quotes")]
    pub quotes_section: Option<Vec<Quote>>, // CHANGE ME 
    #[serde(default, rename = "art")]
    pub artwork: Option<Artwork>,
    #[serde(default)]
    pub table: Option<Table>,
    #[serde(default, rename = "date")]
    pub first_known_use: Option<String>,
    #[serde(default, rename = "shortdef")]
    pub short_definitions: Option<Vec<String>>,
    #[serde(default)]
    pub gram: Option<String>, // DO SOMETHING HERE
    #[serde(default, rename = "artl")]
    pub artwork_learners: Option<ArtworkLearners>,
}
