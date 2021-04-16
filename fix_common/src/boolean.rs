
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Boolean {
    Y,
    N,
}
