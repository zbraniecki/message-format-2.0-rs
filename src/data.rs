use std::collections::HashMap;

pub struct PHTypeAttributes {
    enumerated: bool,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum PlaceholderType {
    UNKNOWN, // from Google protobuf style guide, but is this necessary? I think not
    GENDER,
    PLURAL,
    OTHER(String) // let this be open-ended for all the things we know and
                  // all the future needs we can't predict.
}

// return a map with pre-defined meta-information about common PlaceholderTypes.
// I think this is useful, esp for an l10n/TMS system.  But I haven't decided
// if/how it relates strictly to message formatting itself, so unil then,
// this will actually be unused.  If this is useful, then the user can extend
//  this map to support whatever custom PH types that the user chooses to use
// in `Placeholder.OTHER(String)`.
pub fn ph_type_attrs_map() -> HashMap<PlaceholderType, PHTypeAttributes> {
    let mut m = HashMap::new();
    m.insert(
        PlaceholderType::GENDER,
        PHTypeAttributes{ enumerated: true }
    );
    m.insert(
        PlaceholderType::PLURAL,
        PHTypeAttributes{ enumerated: true }
    );
    m
}

pub struct Placeholder {
    // id & name for PH, used for val interpolation in the formatted string.
    // Let the user decide whether this should be unique or shared
    // across multiple PH instances within the same message.
    // Ex: if PH represents a product name, then maybe you want to
    // call it PRODUCT_NAME everywhere that same string is reoccurs
    // in the message.
    // Ex: if you have multiple inline <span> tags that need to be
    // turned into placeholders, then you might want to use SPAN1,
    // SPAN2, ... to indicate that the contents may very well differ.
    // and <b> and <i> tags may just all be B and I because they are
    // semantically same, and therefore interchangeable.
    id: String,

    // type of the PH.
    // See notes for PlaceholderType for nuances of PH types.
    ph_type: PlaceholderType,

    // a user-supplied text representation of the PH, if available.
    // For PHs that are created by the user (or user's l10n tool),
    // taking a source document, parsing text units out of the doc file format, 
    // and replacing inline non-translatable content in each TU with a PH,
    // we already know the text that the PH is "holding the place" for.
    // If not present, then the value must be present in the map 
    // `SingleMessage.ph_vals` that is keyed by this PH's `Placeholder.id`.
    default_text_val: Option<String>,
}

//#[derive(Hash, Eq)]
pub struct PHValsMap {
    map: HashMap<String, String>,
}

pub struct TextPart {
    text: String,
}

pub enum PatternPart {
    TEXTPART(TextPart),
    PLACEHOLDER(Placeholder),
}

pub struct MessagePattern {
    parts: Vec<PatternPart>
}

pub struct SingleMessage {
    // unique id for the SingleMessage, globally unique.
    id: String,

    locale: String,
    pattern: MessagePattern,
    ph_vals: PHValsMap, // type of value should prob be Any
}

pub struct MessageGroup {
    id: String,
    messages: HashMap<PHValsMap, SingleMessage>,
}

pub enum MessageType {
    SINGLE(SingleMessage),
    GROUP(MessageGroup)
}

pub struct TextUnit {
    src: MessageType,
    tgt: MessageType,
}


//
// unit tests
// 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_message() {
        let msg1 = SingleMessage {
            id: String::from("msg1"),
            locale: String::from("en"),
            pattern: MessagePattern{ 
                parts: vec![
                    PatternPart::TEXTPART(TextPart{ text: String::from("No items selected.") }),
                ],
            },
            ph_vals: PHValsMap{ map: {
                let mut m = HashMap::default();
                m.insert(String::from("COUNT"), String::from("=0"));
                m
            }},
        };
        let msg2 = SingleMessage {
            id: String::from("msg1"),
            locale: String::from("en"),
            pattern: MessagePattern{ 
                parts: vec![
                    PatternPart::PLACEHOLDER(Placeholder{
                        id: String::from("COUNT"),
                        ph_type: PlaceholderType::PLURAL,
                        default_text_val: Option::None,
                     }),
                    PatternPart::TEXTPART(TextPart{ text: String::from(" item selected.") }),
                ],
            },
            ph_vals: PHValsMap{ map: {
                let mut m = HashMap::default();
                m.insert(String::from("COUNT"), String::from("ONE"));
                m
            }},
        };
        let msg3 = SingleMessage {
            id: String::from("msg2"),
            locale: String::from("en"),
            pattern: MessagePattern{ 
                parts: vec![
                    PatternPart::PLACEHOLDER(Placeholder{
                        id: String::from("COUNT"),
                        ph_type: PlaceholderType::PLURAL,
                        default_text_val: Option::None,
                     }),
                    PatternPart::TEXTPART(TextPart{ text: String::from(" items selected.") }),
                ],
            },
            ph_vals: PHValsMap{ map: {
                let mut m = HashMap::default();
                m.insert(String::from("COUNT"), String::from("OTHER"));
                m
            }},
        };

        let msgs = vec![msg1, msg2, msg3];

        // let msg_ids: Vec<String> =
        //     msgs.iter().map(|&m| m.id.clone()).collect();

        // let msg_ph_vals_vec: Vec<PHValsMap> = 
        //     msgs.iter().map(|&m| m.ph_vals.clone()).collect();

        // let mut msgs_grp_map: HashMap<PHValsMap, SingleMessage> = HashMap::new();
        // for msg in msgs {
        //     let ph_vals = &msg.ph_vals.clone();
        //     msgs_grp_map.insert(ph_vals, msg);
        // }
        
    }
}