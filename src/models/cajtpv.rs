use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Cajtpv {
    #[serde(rename = "cajtpv")]
    id: String,
    nombre: String,
}

impl Cajtpv {
    pub fn new(id: String, nombre: String) -> Self {
        Self { id, nombre }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_nombre(&mut self, nombre: String) {
        self.nombre = nombre;
    }
}
