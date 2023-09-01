use std::error::Error;
use std::fmt;

// A generic error type for Tiptap
#[derive(PartialEq)]
pub enum TiptapError {
    TypeNotFound { type_name: Option<String> },
    // You could add more error types here
}

impl fmt::Display for TiptapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TiptapError::TypeNotFound { type_name } => write!(f, "Type not found: {:?}", type_name),
        }
    }
}

impl fmt::Debug for TiptapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for TiptapError {}

impl From<TypeNotFound> for TiptapError {
    fn from(err: TypeNotFound) -> Self {
        TiptapError::TypeNotFound {
            type_name: err.type_name,
        }
    }
}

struct TypeNotFound {
    type_name: Option<String>,
}

impl TypeNotFound {
    fn new(type_name: Option<String>) -> TypeNotFound {
        TypeNotFound { type_name }
    }
}
