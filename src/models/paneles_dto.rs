use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PanelDTO {
    pub cajtpv: String,
    pub panel: String,
    pub nombre: String,
}

pub struct ListOfPanelsDTO(pub Vec<PanelDTO>);
