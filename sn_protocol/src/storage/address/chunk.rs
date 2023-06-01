// Copyright 2023 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use serde::{Deserialize, Serialize};
use std::hash::Hash;
use xor_name::XorName;

/// Address of a Chunk
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Debug)]
pub struct ChunkAddress(XorName);

impl ChunkAddress {
    /// Creates a new ChunkAddress.
    pub fn new(xor_name: XorName) -> Self {
        Self(xor_name)
    }

    /// Returns the name.
    pub fn name(&self) -> &XorName {
        &self.0
    }
}
