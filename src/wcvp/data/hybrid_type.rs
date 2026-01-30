#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum HybridType {
  #[serde(rename = "+")]
  GraftChimera,
  #[serde(rename = "×")]
  Hybrid,
}
