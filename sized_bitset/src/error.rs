use std::array::TryFromSliceError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error(transparent)]
    FromSlice {
        #[from]
        source: TryFromSliceError,
    },
    #[error("invalid character: {0})")]
    FromStr(String),
}
