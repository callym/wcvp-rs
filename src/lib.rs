#![feature(string_remove_matches, nonpoison_rwlock, sync_nonpoison)]

mod error;
mod wcvp;

pub use error::Error;
pub use wcvp::{Climate, Data, HybridType, Metadata, Statistics, Status, TaxonRank, Version, Wcvp};

#[cfg(test)]
mod test {
  use super::*;

  #[tokio::test]
  async fn check_length() -> Result<(), crate::Error> {
    let path = env!("CARGO_MANIFEST_DIR");
    let file = Wcvp::from_file(&format!("{path}/data/wcvp.zip")).await?;

    let metadata = file.metadata();

    assert_eq!(metadata.rows, file.len() as _);

    Ok(())
  }
}
