#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub enum HybridType {
  #[serde(rename = "+")]
  GraftChimera,
  #[serde(rename = "×")]
  Hybrid,
}
