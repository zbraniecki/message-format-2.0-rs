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
    // `Message.ph_vals` that is keyed by this PH's `Placeholder.id`.
    default_text_val: Option<String>,
}

pub struct TextPart {
    text: String,
}

pub enum PatternPart {
    TextPart(TextPart),
    Placeholder(Placeholder),
}

pub struct MessagePattern {
    text_parts: Vec<PatternPart>
}

pub struct Message {
    // unique id for the Message, globally unique.
    id: String,

    locale: String,
    pattern: MessagePattern,
    ph_vals: HashMap<String, String>,
}