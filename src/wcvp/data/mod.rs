mod climate;
mod hybrid_type;
mod status;
mod taxon_rank;

pub use climate::Climate;
pub use hybrid_type::HybridType;
pub use status::Status;
pub use taxon_rank::TaxonRank;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Data {
  /// World Checklist of Vascular Plants (WCVP) identifier
  #[serde(rename = "plant_name_id")]
  pub id: u32,
  /// International Plant Name Index (IPNI) identifier.
  ///
  /// Missing values indicate that the name has not been matched with a name
  /// in IPNI or is missing from IPNI.
  pub ipni_id: Option<String>,
  /// The level in the taxonomic hierarchy where the taxon name fits.
  ///
  /// Some infraspecific names are unranked and will have no value in this column.
  pub taxon_rank: TaxonRank,
  /// Indication of nomenclatural status and taxonomic opinion re the name: see details in main text.
  ///
  /// Names with status ‘Provisionally Accepted’ are unplaced names that have synonyms, following the GBIF classification and only used within the Darwin Core Archive downlaod.
  pub taxon_status: Status,
  /// The name of the family to which the taxon belongs.
  ///
  /// (The highest rank at which names are presented in WCVP).
  pub family: String,
  /// Indication of hybrid status at genus level:
  ///
  /// + indicates a graft-chimaera and × indicates a hybrid.
  pub genus_hybrid: Option<HybridType>,
  /// The name of the genus to which the record refers.
  pub genus: String,
  /// Indication of hybrid status at species level:
  ///
  /// + indicates a graft-chimaera and × indicates a hybrid.
  pub species_hybrid: Option<HybridType>,
  /// The species epithet which is combined with the genus name to make a binomial name for a species.
  ///
  /// Empty when the taxon name is at the rank of genus.
  pub species: Option<String>,
  /// The taxonomic rank of the infraspecific epithet.
  ///
  /// Empty where the taxon name is species rank or higher.
  ///
  /// For more information, see the
  /// [International Code of Nomenclature for algae, fungi and plants](https://www.iapt-taxon.org/nomen/main.php)
  pub infraspecific_rank: Option<String>,
  /// The infraspecific epithet which is combined with a binomial to make a trinomial name at infraspecific rank.
  ///
  /// Empty when taxon name is at species rank or higher.
  pub infraspecies: Option<String>,
  /// The author of the basionym.
  ///
  /// Empty when there is no basionym.
  pub parenthetical_author: Option<String>,
  /// The author or authors who published the scientific name.
  ///
  /// Missing values indicate instances where authorship is non-applicable (i.e. autonyms) or unknown.
  pub primary_author: Option<String>,
  /// The author or authors of the book where the scientific name is first published when different from the primary author.
  ///
  /// Missing values indicate instances where the primary author is also the author of the book or non-applicable (i.e. autonyms).
  pub publication_author: Option<String>,
  /// The journal, book or other publication in which the taxon name was effectively published.
  ///
  /// (Abbreviated for brevity)
  ///
  /// Missing values indicate instances where publication details are unknown or non-applicable (i.e. autonyms).
  pub place_of_publication: Option<String>,
  /// The volume and page numbers of the original publication of the taxon name, where "5(6): 36" is volume 5, issue 6, page 36.
  ///
  /// (Not all volumes include issue number)
  ///
  /// Missing values indicate instances where publication details are unknown or non-applicable (i.e. autonyms).
  pub volume_and_page: Option<String>,
  /// The year of publication of the name, enclosed in parentheses.
  ///
  /// Missing values indicate instances where publication details are unknown or non-applicable (i.e. autonyms).
  pub first_published: Option<String>,
  /// Remarks on the nomenclature.
  ///
  /// (Preceded by a comma and space (", ") for easy concatenation.)
  pub nomenclatural_remarks: Option<String>,
  /// The geographic distribution of the taxon (for names of species rank or below): a generalised statement in narrative form.
  ///
  /// See [here](https://powo.science.kew.org/about-wcvp#geographicaldistribution) for details
  pub geographic_area: Option<String>,
  /// The lifeform (or lifeforms) of the taxon.
  /// Terms refer to a modified verison of the Raunkiær system. Missing values if unknown.
  ///
  /// See [here](https://powo.science.kew.org/about-wcvp#lifeforms) for a glossary of terms used
  pub lifeform_description: Option<String>,
  /// Habitat type of the taxon, derived from published habitat information.
  pub climate_description: Option<Climate>,
  /// Concatenation of genus with species and, where applicable, infraspecific epithets to make a binomial or trinomial name.
  pub taxon_name: String,
  /// Concatenation of parenthetical and primary authors.
  ///
  /// Missing values indicate instances where authorship is unknown or non-applicable (e.g. autonyms).
  pub taxon_authors: Option<String>,
  /// The ID of the accepted name of this taxon. Where the taxon_status is "Accepted", this will be identical to the plant_name_id value.
  ///
  /// May be empty if taxon status is unplaced, ilegitimate, or in some cases where the accepted name is not a vascular plant (e.g. a moss, alga or animal).
  pub accepted_plant_name_id: Option<String>,
  /// ID of the original name that taxon_name was derived from. If there is a parenthetical author it is a basionym.
  ///
  /// If there is a replaced synonym author it is a replaced synonym. If empty there have been no name changes.
  pub basionym_plant_name_id: Option<String>,
  /// The author or authors responsible for publication of the replaced synonym.
  ///
  /// Empty when the name is not a replacement name based on another name.
  pub replaced_synonym_author: Option<String>,
  /// The synonym type - TRUE if homotypic synonym, otherwise NA.
  ///
  /// For more information, see the [International Code of Nomenclature for algae, fungi and plants](https://www.iapt-taxon.org/nomen/main.php)
  pub homotypic_synonym: Option<String>,
  /// ID for the parent genus or parent species of an accepted species or infraspecific name.
  ///
  /// Empty for non accepted names or where the parent has not yet been calculated.
  pub parent_plant_name_id: Option<String>,
  /// identifier required to look up the name directly in Plants of the World Online (Powo)
  pub powo_id: String,
  /// parents of hybrid
  pub hybrid_formula: Option<String>,
  /// Flag indicating whether the family to which the taxon belongs has been peer reviewed.
  #[serde(deserialize_with = "deserialize_bool")]
  pub reviewed: bool,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  let s: &str = serde::de::Deserialize::deserialize(deserializer)?;

  match s {
    "Y" => Ok(true),
    "" | "N" => Ok(false),
    _ => Err(serde::de::Error::unknown_variant(s, &["Y", "N"])),
  }
}
