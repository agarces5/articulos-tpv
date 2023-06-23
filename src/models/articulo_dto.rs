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
