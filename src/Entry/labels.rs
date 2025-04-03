//! ## 2.7 LABELS
//! A label provides a brief note on the grammatical function, subject area, register, regional usage, or capitalization of a headword, whether generally or in a particular sense.
//!
//! [API Documentation ↗︎]
//!
//! [API Documentation ↗︎]: https://dictionaryapi.com/products/json#sec-2.labels

/// ## 2.7.1 FUNCTIONAL LABEL: `FL`
/// The functional label describes the grammatical function of a headword or undefined entry word, such as "noun" or "adjective".
///
/// ### Hierarchical Context
/// Occurs as a top-level member of the dictionary entry, where it describes the hw in the preceding hwi. Also occurs within uros, where it describes the preceding ure.
///
/// ### Display Guidance
/// Typically rendered in italics
///
/// [API Documentation ↗︎]
///
/// [API Documentation ↗︎]: https://dictionaryapi.com/products/json#sec-2.fl
pub type FunctionalLabel = String;
pub type GeneralLabel = String;
pub type ParenthesizedSubjectStatusLabel = String;
pub type SubjectStatusLabel = String;
pub type SenseSpecificInflectionPluralLabel = String;
pub type SenseSpecificGrammaticalLabel = String;
