use std::{collections::HashMap, fmt::Display, ops::Deref, vec};

use serde::{Deserialize, Serialize};

use crate::{context::filters::Filter, models::articulo_dto::ListArticuloDTO};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
/// id: u32
/// nombre: String
/// familia: String
/// detalles: Vec<Detail>, lista de cajas y precios
pub struct Articulo {
    pub id: u32,
    pub nombre: String,
    pub familia: String,
    pub detalles: Vec<Detail>,
}
impl Articulo {
    /// Obtiene el primer detalle del articulo si lo hubiera
    pub fn some_detail(&self) -> Option<Detail> {
        self.detalles.first().cloned()
    }
}
/// cajtpv: String,
/// precios: Vec<Precio>, lista de tarifas y precios
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Detail {
    pub cajtpv: String,
    pub precios: Vec<Precio>,
}
/// tipotarifa: String,
/// precio: f64
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Precio {
    pub tipotarifa: String,
    pub precio: f64,
}
impl Display for Precio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\t{}€", self.tipotarifa, self.precio)
    }
}

pub struct ListArticulo(Vec<(u32, Articulo)>);

impl ListArticulo {
    pub fn _new(list: Vec<Articulo>) -> Self {
        let new_list = list.into_iter().map(|art| (art.id, art)).collect();
        ListArticulo(new_list)
    }
    pub fn filter(&self, filter: Filter) -> Vec<Articulo> {
        let cajtpv = filter.cajtpv;
        let familia = filter.familia;
        self.deref()
            .clone()
            .into_iter()
            .filter(|(_id, art)| {
                art.familia == familia
                    && art.detalles.iter().any(|detail| match cajtpv.as_ref() {
                        "all" => true,
                        _ => detail.cajtpv == cajtpv,
                    })
            })
            .map(|(_id, art)| {
                if let Some(detalles) = art.detalles.iter().find(|det| match cajtpv.as_ref() {
                    "all" => true,
                    _ => det.cajtpv == cajtpv,
                }) {
                    Articulo {
                        detalles: vec![detalles.clone()],
                        ..art
                    }
                } else {
                    Articulo::default()
                }
            })
            .collect()
    }
    pub fn _find_by_id(&self, id: u32) -> Option<Articulo> {
        self.iter()
            .find(|(art_id, _art)| art_id == &id)
            .map(|(_id, art)| art.clone())
    }
}

impl From<ListArticuloDTO> for ListArticulo {
    fn from(lista_articulos_dto: ListArticuloDTO) -> Self {
        let lista_articulos_dto = lista_articulos_dto.get();
        // Creamos un mapa para almacenar los datos en el nuevo formato
        let mut map_de_articulos: HashMap<u32, Articulo> = HashMap::new();

        // Recorremos cada elemento de la lista que nos llega
        for articulo_dto in lista_articulos_dto {
            let id = articulo_dto.articulo as u32;
            let nombre = articulo_dto.nombre.clone();
            let familia = articulo_dto.familia.clone();
            let cajtpv = articulo_dto.cajtpv.clone().unwrap_or_default();
            let tipotarifa = articulo_dto.tipotarifa.clone();
            let precio = articulo_dto.precio;

            // Para cada articulo, verificamos si ya existe y si no lo creamos
            let articulo = map_de_articulos.entry(id).or_insert_with(|| Articulo {
                id,
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
                    details.precios.push(Precio {
                        tipotarifa,
                        precio: precio / 10000.0,
                    })
                }
            } else if let (Some(tipotarifa), Some(precio)) = (tipotarifa, precio) {
                // Creamos un nuevo detalle
                let new_detail = Detail {
                    cajtpv,
                    precios: vec![Precio {
                        tipotarifa,
                        precio: precio / 10000.0,
                    }],
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

impl Deref for ListArticulo {
    type Target = Vec<(u32, Articulo)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
