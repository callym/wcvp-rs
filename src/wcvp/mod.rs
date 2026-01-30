use std::{
  collections::HashMap,
  io::{Cursor, Read},
  path::Path,
};

use zip::ZipArchive;

mod data;
mod metadata;
mod statistics;
mod version;

pub use data::{Climate, Data, HybridType, Status, TaxonRank};
pub use metadata::Metadata;
pub use statistics::Statistics;
pub use version::Version;

type DataFields = (HashMap<u32, Data>, HashMap<String, u32>);

#[derive(Debug)]
pub struct Wcvp {
  metadata: Metadata,
  data: HashMap<u32, Data>,
  powo: HashMap<String, u32>,
  statistics: Statistics,
}

impl Wcvp {
  pub async fn from_file(path: impl AsRef<Path>) -> Result<Self, crate::Error> {
    let file = tokio::fs::read(path).await?;
    let mut file = ZipArchive::new(Cursor::new(file))?;

    let metadata = Wcvp::load_metadata(&mut file)?;
    let (data, powo) = Wcvp::load_data(&mut file)?;
    let statistics = Statistics::calculate(data.values());

    Ok(Self {
      metadata,
      data,
      powo,
      statistics,
    })
  }

  pub async fn download() -> Result<Self, crate::Error> {
    let res = reqwest::get("https://sftp.kew.org/pub/data-repositories/WCVP/wcvp.zip").await?;

    let bytes = res.bytes().await?.to_vec();

    let mut file = ZipArchive::new(Cursor::new(bytes))?;

    let metadata = Wcvp::load_metadata(&mut file)?;
    let (data, powo) = Wcvp::load_data(&mut file)?;
    let statistics = Statistics::calculate(data.values());

    Ok(Self {
      metadata,
      data,
      powo,
      statistics,
    })
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  pub fn data(&self) -> impl Iterator<Item = &Data> {
    self.data.values()
  }

  pub fn metadata(&self) -> &Metadata {
    &self.metadata
  }

  pub fn from_id(&self, id: u32) -> Option<&Data> {
    self.data.get(&id)
  }

  pub fn from_powo(&self, powo: impl AsRef<str>) -> Option<&Data> {
    let id = self.powo.get(powo.as_ref())?;

    self.from_id(*id)
  }

  pub fn statistics(&self) -> &Statistics {
    &self.statistics
  }

  fn load_data(file: &mut ZipArchive<Cursor<Vec<u8>>>) -> Result<DataFields, crate::Error> {
    let mut readme = file.by_name("wcvp_names.csv")?;

    let mut buf = Vec::new();
    readme.read_to_end(&mut buf)?;

    let mut csv = csv::ReaderBuilder::new()
      .delimiter(b'|')
      .from_reader(Cursor::new(buf.clone()));

    let mut powo_map = HashMap::new();
    let mut data_map = HashMap::new();

    for result in csv.deserialize() {
      let data: Data = result?;

      powo_map.insert(data.powo_id.clone(), data.id);
      data_map.insert(data.id, data);
    }

    Ok((data_map, powo_map))
  }

  fn load_metadata(file: &mut ZipArchive<Cursor<Vec<u8>>>) -> Result<Metadata, crate::Error> {
    let mut readme = file.by_name("README_WCVP.xlsx")?;

    let mut buf = Vec::new();
    readme.read_to_end(&mut buf)?;

    let readme = calamine::open_workbook_from_rs::<calamine::Xlsx<_>, _>(Cursor::new(buf))?;

    Metadata::try_from(readme)
  }
}
