#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub enum Climate {
  #[serde(rename = "desert and dry shrubland")]
  DesertAndDryShrubland,
  #[serde(rename = "desert or dry shrubland")]
  DesertOrDryShrubland,
  #[serde(rename = "montane tropical")]
  MontaneTropical,
  #[serde(rename = "seasonally dry tropical")]
  SeasonallyDryTropical,
  #[serde(rename = "subalpine or subarctic")]
  SubalpineOrSubarctic,
  #[serde(rename = "subtropical")]
  Subtropical,
  #[serde(rename = "subtropical and tropical")]
  SubtropicalAndTropical,
  #[serde(rename = "subtropical or tropical")]
  SubtropicalOrTropical,
  #[serde(rename = "temperate")]
  Temperate,
  #[serde(rename = "temperate and tropical")]
  TemperateAndTropical,
  #[serde(rename = "temperate, subtropical or tropical")]
  TemperateOrTropicalOrSubtropical,
  #[serde(rename = "wet tropical")]
  WetTropical,
}
