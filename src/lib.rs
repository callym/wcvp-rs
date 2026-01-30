#![feature(string_remove_matches, nonpoison_rwlock, sync_nonpoison)]

mod error;
mod wcvp;

pub use error::Error;
pub use wcvp::{Climate, Data, HybridType, Metadata, Statistics, Status, TaxonRank, Version, Wcvp};
