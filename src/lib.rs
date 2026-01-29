#![feature(string_remove_matches, nonpoison_rwlock, sync_nonpoison)]

mod error;
mod wcvp;

pub use error::Error;
pub use wcvp::{Climate, Data, HybridType, Metadata, Statistics, Status, TaxonRank, Version, Wcvp};

#[tokio::test]
async fn test() -> Result<(), Box<dyn std::error::Error>> {
  use std::path::PathBuf;

  let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  dir.push("data/wcvp.zip");
  dbg!(&dir);

  let mut data = Wcvp::from_file(dir).await?;

  dbg!(data.metadata().unwrap());

  data.statistics()?;

  dbg!(data.from_powo("969084-1"));

  Ok(())
}
