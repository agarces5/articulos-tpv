use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArticuloDTO {
    pub articulo: f64,
    pub nombre: String,
    pub familia: String,
    pub cajtpv: Option<String>,
    pub tipotarifa: Option<String>,
    pub precio: Option<f64>,
}

pub struct ListArticuloDTO(Vec<ArticuloDTO>);

impl ListArticuloDTO {
    pub fn new(lista: Vec<ArticuloDTO>) -> Self {
        ListArticuloDTO(lista)
    }
    pub fn get_list(&self) -> &Vec<ArticuloDTO> {
        &self.0
    }
    pub fn get_owned_list(self) -> Vec<ArticuloDTO> {
        self.0
    }
}
