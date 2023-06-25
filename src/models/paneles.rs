use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::paneles_dto::ListOfPanelsDTO;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Panel {
    pub panel: String,
    pub nombre: String,
    pub cajtpv: Vec<String>,
}

pub struct ListOfPanels(Vec<(String, Panel)>);

impl ListOfPanels {
    pub fn new(lista: Vec<(String, Panel)>) -> Self {
        Self(lista)
    }
    pub fn get(&self) -> &Vec<(String, Panel)> {
        &self.0
    }
    pub fn filter(&self, cajtpv: &str) -> Vec<(String, Panel)> {
        self.get()
            .clone()
            .into_iter()
            .filter(|(_id, panel)| panel.cajtpv.iter().any(|caja| caja == cajtpv))
            .collect()
    }
}

impl From<ListOfPanelsDTO> for ListOfPanels {
    fn from(value: ListOfPanelsDTO) -> Self {
        let list_of_panels_dto = value.0;
        let mut map_de_paneles: HashMap<String, Panel> = HashMap::new();

        for panel_dto in list_of_panels_dto {
            let id = panel_dto.panel;
            let nombre = panel_dto.nombre;
            let cajtpv = panel_dto.cajtpv;
            // Creamos una nueva entrada si no existe.
            let panel = map_de_paneles.entry(id.clone()).or_insert(Panel {
                panel: id,
                nombre,
                cajtpv: vec![],
            });
            // Verificamos si ya tiene la caja y si no lo añadimos
            if !panel.cajtpv.iter().any(|caja| caja == &cajtpv) {
                panel.cajtpv.push(cajtpv);
            }
        }

        let mut lista_ordenada_paneles: Vec<_> = map_de_paneles.into_iter().collect();
        lista_ordenada_paneles.sort_by_key(|(id, _panel)| id.clone());
        ListOfPanels::new(lista_ordenada_paneles)
    }
}
