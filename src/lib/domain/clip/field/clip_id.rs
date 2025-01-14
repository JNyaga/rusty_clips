use crate::data::Dbid;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Constructor)]
pub struct ClipId(Dbid);

impl ClipId {
    pub fn into_inner(self) -> Dbid {
        self.0
    }
}

//convert from Dbid to ClipId
impl From<Dbid> for ClipId {
    fn from(id: Dbid) -> Self {
        Self(id)
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(Dbid::nil())
    }
}
