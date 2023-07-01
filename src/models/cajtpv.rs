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

    pub fn id(&self) -> String {
        self.id.to_owned()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn nombre(&self) -> String {
        self.nombre.to_owned()
    }

    pub fn set_nombre(&mut self, nombre: String) {
        self.nombre = nombre;
    }
}
