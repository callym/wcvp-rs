#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub enum Status {
  Accepted,
  #[serde(rename = "Artificial Hybrid")]
  ArtificialHybrid,
  Illegitimate,
  Invalid,
  #[serde(rename = "Local Biotype")]
  LocalBiotype,
  Misapplied,
  Orthographic,
  Synonym,
  Unplaced,
  #[serde(rename = "Provisionally Accepted")]
  ProvisionallyAccepted,
}
