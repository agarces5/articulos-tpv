use std::{collections::HashMap, vec};

use serde::{Deserialize, Serialize};

use crate::models::articulo_dto::ListArticuloDTO;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Articulo {
    pub articulo: u32,
    pub nombre: String,
    pub familia: String,
    pub detalles: Vec<Detail>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Detail {
    pub cajtpv: String,
    pub precios: Vec<Precio>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Precio {
    pub tipotarifa: String,
    pub precio: f64,
}

pub struct ListArticulo(Vec<(u32, Articulo)>);

impl ListArticulo {
    pub fn new(list: Vec<(u32, Articulo)>) -> Self {
        ListArticulo(list)
    }
    pub fn get(&self) -> &Vec<(u32, Articulo)> {
        &self.0
    }
    pub fn filter(&self, cajtpv: &str, familia: &str) -> Vec<(u32, Articulo)> {
        self.get()
            .clone()
            .into_iter()
            .filter(|(_id, art)| {
                art.familia == familia && art.detalles.iter().any(|detail| detail.cajtpv == cajtpv)
            })
            .collect()
    }
}

impl From<ListArticuloDTO> for ListArticulo {
    fn from(lista_articulos_dto: ListArticuloDTO) -> Self {
        let lista_articulos_dto = lista_articulos_dto.get_owned_list();
        // Creamos un mapa para almacenar los datos en el nuevo formato
        let mut map_de_articulos: HashMap<u32, Articulo> = HashMap::new();

        // Recorremos cada elemento de la lista que nos llega
        for articulo_dto in lista_articulos_dto {
            let id = articulo_dto.articulo as u32;
            let nombre = articulo_dto.nombre;
            let familia = articulo_dto.familia;
            let cajtpv = articulo_dto.cajtpv.unwrap_or_default();
            let tipotarifa = articulo_dto.tipotarifa;
            let precio = articulo_dto.precio;

            // Para cada articulo, verificamos si ya existe y si no lo creamos
            let articulo = map_de_articulos.entry(id).or_insert_with(|| Articulo {
                articulo: id,
                nombre,
                familia,
                detalles: vec![],
            });

            // Verificamos si ya hay una cajtpv
            if let Some(details) = articulo
                .detalles
                .iter_mut()
                .find(|detail| detail.cajtpv == cajtpv)
            {
                // Agregamos un nuevo precio y tarifa
                if let (Some(tipotarifa), Some(precio)) = (tipotarifa, precio) {
                    details.precios.push(Precio { tipotarifa, precio })
                }
            } else if let (Some(tipotarifa), Some(precio)) = (tipotarifa, precio) {
                // Creamos un nuevo detalle
                let new_detail = Detail {
                    cajtpv,
                    precios: vec![Precio { tipotarifa, precio }],
                };
                // Agregamos el nuevo detalle al articulo
                articulo.detalles.push(new_detail);
            }
        }
        let mut sorted_list: Vec<_> = map_de_articulos.into_iter().collect();
        sorted_list.sort_by_key(|(clave, _)| *clave);
        ListArticulo(sorted_list)
    }
}
