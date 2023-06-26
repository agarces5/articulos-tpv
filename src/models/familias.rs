use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Familia {
    familia: String,
    nombre: String,
}

impl Familia {
    pub fn nombre(&self) -> &str {
        &self.nombre
    }
    pub fn _filter(&self) -> bool {
        true
    }

    pub fn familia(&self) -> &str {
        self.familia.as_ref()
    }
}
