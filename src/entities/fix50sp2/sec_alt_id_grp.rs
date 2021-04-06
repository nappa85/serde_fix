
use serde::{Serialize, Deserialize};

use crate::entities::RepeatingValues;

use super::instrument::SecurityIDSource;

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct SecAltIDGrp {
    #[serde(rename = "454")]
    pub inner: RepeatingValues<SecAltID>,
}

impl AsRef<Vec<SecAltID>> for SecAltIDGrp {
    fn as_ref(&self) -> &Vec<SecAltID> {
        self.inner.as_ref()
    }
}

impl AsMut<Vec<SecAltID>> for SecAltIDGrp {
    fn as_mut(&mut self) -> &mut Vec<SecAltID> {
        self.inner.as_mut()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct SecAltID {
    #[serde(rename = "455")]
    pub security_alt_id: Option<String>,
    #[serde(rename = "456")]
    pub security_alt_id_source: Option<SecurityIDSource>,
}
