#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Invalid version: {0}")]
  InvalidVersion(u32),

  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  Zip(#[from] zip::result::ZipError),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Calamine(#[from] calamine::Error),
  #[error(transparent)]
  CalamineXlsx(#[from] calamine::XlsxError),
  #[error(transparent)]
  Csv(#[from] csv::Error),
  #[error(transparent)]
  TimeParse(#[from] time::error::Parse),
}
