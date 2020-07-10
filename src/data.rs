use std::collections::HashMap;

pub enum PlaceholderType {
    UNKNOWN, // from Google protobuf style guide, but is this necessary? I think not
    GENDER,
    PLURAL,
    OTHER(String) // let this be open-ended for all the things we know and
                  // all the future needs we can't predict.
}

pub struct Placeholder {
    // unique id for the PH.
    // Conceptually, this is globally unique. But it is up to user,
    // at what scope this is guaranteed to be unique, but obviously
    // should be unique at least at the message level.
    id: String,

    // name for PH, used for val interpolation in the formatted string. 
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
    name: String,

    // type of the PH.
    // See notes for PlaceholderType for nuances of PH types.
    ph_type: PlaceholderType,

    // a user-supplied text representation of the PH, if available.
    // For PHs that are created by the user (or user's l10n tool),
    // taking a source document, parsing text units out of the doc file format, 
    // and replacing inline non-translatable content in each TU with a PH,
    // we already know the text that the PH is "holding the place" for.
    // If not present, then the value must be present in the map 
    // `Message.ph_vals` that is keyed by this PH's `Placeholder.name`.
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