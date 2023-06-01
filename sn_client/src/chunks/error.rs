// Copyright 2023 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use std::io;
use thiserror::Error;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

/// Internal error.
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    SelfEncryption(#[from] self_encryption::Error),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Serialisation(#[from] Box<bincode::ErrorKind>),

    #[error("Cannot store empty file.")]
    EmptyFileProvided,

    #[error(
        "Not enough bytes ({size}) for self-encryption, at least {minimum} bytes needed. \
        Try storing it as a SmallFile."
    )]
    TooSmallForSelfEncryption {
        /// Number of bytes
        size: usize,
        /// Minimum number of bytes for self-encryption
        minimum: usize,
    },

    #[error(
        "You might need to pad the `SmallFile` contents and then store it as a `LargeFile`, \
        as the encryption has made it slightly too big ({0} bytes)"
    )]
    SmallFilePaddingNeeded(usize),

    #[error(
        "The provided bytes ({size}) is too large to store as a `SmallFile` which maximum can be \
        {maximum}. Store as a LargeFile instead."
    )]
    TooLargeAsSmallFile {
        /// Number of bytes
        size: usize,
        /// Maximum number of bytes for a `SmallFile`
        maximum: usize,
    },

    #[error("Not all chunks were retrieved, expected {expected}, retrieved {retrieved}.")]
    NotEnoughChunksRetrieved {
        /// Number of Chunks expected to be retrieved
        expected: usize,
        /// Number of Chunks retrieved
        retrieved: usize,
    },

    #[error("Not all data was chunked, expected {expected}, but we have {chunked}.)")]
    NotAllDataWasChunked {
        /// Number of Chunks expected to be generated
        expected: usize,
        /// Number of Chunks generated
        chunked: usize,
    },
}
