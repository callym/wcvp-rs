use std::collections::HashMap;

use crate::{Data, wcvp::data::TaxonRank};

#[derive(Debug, Clone)]
pub struct Statistics {
  taxon_ranks: Stat<TaxonRank>,
  lifeform_descriptions: Stat<String>,
  geographic_area: Stat<String>,
}

impl Statistics {
  pub fn calculate<'a>(iter: impl Iterator<Item = &'a Data>) -> Self {
    let mut taxon_ranks = Stat::new();
    let mut lifeform_descriptions = Stat::new();
    let mut geographic_area = Stat::new();

    for data in iter {
      taxon_ranks.add(data.taxon_rank);

      if let Some(desc) = data.lifeform_description.clone() {
        lifeform_descriptions.add(desc);
      }

      if let Some(area) = data.geographic_area.clone() {
        geographic_area.add(area);
      }
    }

    Statistics {
      taxon_ranks,
      lifeform_descriptions,
      geographic_area,
    }
  }

  pub fn taxon_ranks(&self) -> impl Iterator<Item = (TaxonRank, u32)> {
    self.taxon_ranks.clone().into_iter()
  }

  pub fn lifeform_descriptions(&self) -> impl Iterator<Item = (String, u32)> {
    self.lifeform_descriptions.clone().into_iter()
  }

  pub fn geographic_area(&self) -> impl Iterator<Item = (String, u32)> {
    self.geographic_area.clone().into_iter()
  }
}

#[derive(Debug, Clone)]
struct Stat<T: Clone + Eq + std::hash::Hash> {
  map: HashMap<T, u32>,
}

impl<T: Clone + Eq + std::hash::Hash> Stat<T> {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
    }
  }

  pub fn add(&mut self, key: T) {
    self.map.entry(key).and_modify(|i| *i += 1).or_insert(1);
  }
}

impl<T: Clone + Eq + std::hash::Hash> IntoIterator for Stat<T> {
  type IntoIter = <Vec<(T, u32)> as IntoIterator>::IntoIter;
  type Item = (T, u32);

  fn into_iter(self) -> Self::IntoIter {
    let mut vec = self
      .map
      .iter()
      .map(|(v, i)| (v.clone(), *i))
      .collect::<Vec<_>>();

    vec.sort_by_key(|(_, i)| *i);
    vec.reverse();

    vec.into_iter()
  }
}
