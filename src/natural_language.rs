//! Analyze natural language text and deduce its language-specific metadata.

use crate::foundation;

/// The languages that the Natural Language framework supports.
pub type NLLanguage = foundation::NSString;

/// Constants for the tag schemes specified when initializing a linguistic tagger.
pub type NLTagScheme = foundation::NSString;

mod enums;
mod nl_language;
mod nl_language_recognizer;
mod nl_tag_scheme;
mod nl_tokenizer;

pub use enums::*;
pub use nl_language::*;
pub use nl_language_recognizer::*;
pub use nl_tag_scheme::*;
pub use nl_tokenizer::*;
