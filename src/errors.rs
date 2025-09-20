use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("This is not an Object, cannot be flattened!")]
    NotAnObject,

    #[error("This should be a Value")]
    NotAValue,

    #[error("The property is not valid")]
    InvalidProperty,

    #[error("mixed type array")]
    MixedTypeArray,

    #[error("This should be an Object or an Array")]
    InvalidType,

    #[error("Unknown Error")]
    Unspecified,

    #[error("JSON format error")]
    FormatError,
}
