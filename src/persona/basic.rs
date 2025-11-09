use serde::{Deserialize, Serialize};

use crate::persona::PType;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Basic {
    name: String,
    description: String,
}

impl From<Basic> for PType {
    fn from(basic: Basic) -> Self {
        PType::Basic(basic)
    }
}

impl Basic {
    pub fn new(name: &str, description: &str) -> PType {
        Basic {
            name: name.to_string(),
            description: description.to_string(),
        }
        .into()
    }

    pub fn load_from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}
