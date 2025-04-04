///# 2.10.10 Parenthesized Sense Sequence: pseq
///
///The parenthesized sense sequence groups together senses whose sense numbers form a sequence of parenthesized numbers.
///
///## Hierarchical Context
///
///Occurs as an element in an sseq array.
///
///## Display Guidance
///
///If you are generating sense numbers for sense elements in a pseq sequence, put parentheses around the number. For example, the second sense in a sequence should have "(2)" as its sense number.
///
///If you are instead using the sn to display the sense number, it will already contain the parentheses.
///
///### Data Model
///
///array consisting of one or more sense elements and an optional bs element.
/// 
/// [API Docs](https://dictionaryapi.com/products/json#sec-2.pseq)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParenSenseSequence {

}