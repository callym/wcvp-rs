#[derive(Debug, Clone, Copy, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum TaxonRank {
  Convariety,
  Form,
  Genus,
  // proles,
  Species,
  Subform,
  Subspecies,
  Subvariety,
  Variety,
  //
  #[serde(rename = "nothosubsp.")]
  NothoSubspecies,
  #[serde(rename = "nothovar.")]
  NothoVariety,
  #[serde(rename = "nothof.")]
  Nothoform,
  //
  #[serde(rename = "subsubsp.")]
  Subsubspecies,
  //
  #[serde(rename = "grex")]
  Grex,
  #[serde(rename = "mut.")]
  Mutation,
  #[serde(rename = "modif.")]
  Modif,
  #[serde(rename = "agamosp.")]
  Agamosp,
  #[serde(rename = "group")]
  Group,
  //
  /// I think this means a natural sport - a historical term?
  #[serde(rename = "lusus")]
  Lusus,
  #[serde(rename = "sublusus")]
  Sublusus,
  /// I think this means "race" - not really used?
  #[serde(rename = "proles")]
  Proles,
  #[serde(rename = "subproles")]
  Subproles,
  #[serde(rename = "stirps")]
  Stirps,
  #[serde(rename = "monstr.")]
  Monstr,
  /// Guessing a small population?
  #[serde(rename = "microgène")]
  Microgene,
  #[serde(rename = "micromorphe")]
  Micromorphe,
  #[serde(rename = "microf.")]
  Microf,
  #[serde(rename = "provar.")]
  Provar,
  #[serde(rename = "subap.")]
  Subap,
  #[serde(rename = "subspecioid")]
  Subspecioid,
  #[serde(rename = "nid")]
  Nid,
  #[serde(rename = "psp.")]
  Psp,
  #[serde(rename = "ecas.")]
  Ecas,
  #[serde(rename = "positio")]
  Positio,
  /// So we don't have to use an Option
  #[serde(rename = "")]
  NotRanked,
}
