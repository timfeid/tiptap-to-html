use std::error::Error;
use std::fmt;

// A generic error type for Tiptap
#[derive(PartialEq)]
pub enum ProseMirrorError {
    TypeNotFound { type_name: Option<String> },
    // You could add more error types here
}

impl fmt::Display for ProseMirrorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProseMirrorError::TypeNotFound { type_name } => {
                write!(f, "Type not found: {:?}", type_name)
            }
        }
    }
}

impl fmt::Debug for ProseMirrorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for ProseMirrorError {}

impl From<TypeNotFound> for ProseMirrorError {
    fn from(err: TypeNotFound) -> Self {
        ProseMirrorError::TypeNotFound {
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
