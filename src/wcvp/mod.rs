use std::{
  collections::HashMap,
  io::{Cursor, Read},
  path::Path,
  sync::nonpoison::RwLock,
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

#[derive(Debug)]
pub struct Wcvp {
  file: RwLock<ZipArchive<Cursor<Vec<u8>>>>,
  metadata: Option<Metadata>,
  data: HashMap<u32, Data>,
  powo: HashMap<String, u32>,
  statistics: Option<Statistics>,
}

impl Wcvp {
  pub async fn from_file(path: impl AsRef<Path>) -> Result<Self, crate::Error> {
    let file = tokio::fs::read(path).await?;

    Ok(Self {
      file: RwLock::new(ZipArchive::new(Cursor::new(file))?),
      metadata: None,
      data: HashMap::new(),
      powo: HashMap::new(),
      statistics: None,
    })
  }

  pub async fn download() -> Result<Self, crate::Error> {
    let res = reqwest::get("https://sftp.kew.org/pub/data-repositories/WCVP/wcvp.zip").await?;

    let bytes = res.bytes().await?.to_vec();

    Ok(Self {
      file: RwLock::new(ZipArchive::new(Cursor::new(bytes))?),
      metadata: None,
      data: HashMap::new(),
      powo: HashMap::new(),
      statistics: None,
    })
  }

  pub fn metadata(&mut self) -> Result<Metadata, crate::Error> {
    if let Some(metadata) = self.metadata.clone() {
      return Ok(metadata);
    }

    let mut file = self.file.write();
    let mut readme = file.by_name("README_WCVP.xlsx")?;

    let mut buf = Vec::new();
    readme.read_to_end(&mut buf)?;

    let readme = calamine::open_workbook_from_rs::<calamine::Xlsx<_>, _>(Cursor::new(buf))?;

    let metadata = Metadata::try_from(readme)?;

    self.metadata = Some(metadata.clone());

    Ok(metadata)
  }

  pub fn data(&mut self) -> Result<impl Iterator<Item = &Data>, crate::Error> {
    if !self.data.is_empty() {
      return Ok(self.data.values());
    }

    let mut file = self.file.write();
    let mut readme = file.by_name("wcvp_names.csv")?;

    let mut buf = Vec::new();
    readme.read_to_end(&mut buf)?;

    let mut csv = csv::ReaderBuilder::new()
      .delimiter(b'|')
      .from_reader(Cursor::new(buf.clone()));

    for result in csv.deserialize() {
      let data: Data = result?;

      self.powo.insert(data.powo_id.clone(), data.id);
      self.data.insert(data.id, data);
    }

    Ok(self.data.values())
  }

  pub fn from_id(&self, id: u32) -> Option<&Data> {
    self.data.get(&id)
  }

  pub fn from_powo(&self, powo: impl AsRef<str>) -> Option<&Data> {
    let id = self.powo.get(powo.as_ref())?;

    self.from_id(*id)
  }

  pub fn statistics(&mut self) -> Result<Statistics, crate::Error> {
    if let Some(stats) = self.statistics.clone() {
      return Ok(stats);
    }

    let stats = Statistics::calculate(self.data()?);

    self.statistics = Some(stats.clone());

    Ok(stats)
  }
}
