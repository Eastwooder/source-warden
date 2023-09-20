use serde::{Deserialize, Serialize};

use super::actions::Action;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pass(Action);
