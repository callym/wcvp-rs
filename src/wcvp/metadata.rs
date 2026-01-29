use std::io::Cursor;

use calamine::DataRef;
use time::{Date, macros::format_description};

use crate::Version;

#[derive(Debug, Clone)]
pub struct Metadata {
  pub version: Version,
  pub extracted: Date,
}

impl TryFrom<calamine::Xlsx<Cursor<Vec<u8>>>> for Metadata {
  type Error = crate::Error;

  fn try_from(mut value: calamine::Xlsx<Cursor<Vec<u8>>>) -> Result<Self, Self::Error> {
    let mut sheet = value.worksheet_cells_reader("README")?;

    let mut version = None;
    let mut extracted = None;

    while let Ok(Some(cell)) = sheet.next_cell() {
      let data = match cell.get_value() {
        DataRef::String(s) => (*s).as_str(),
        DataRef::SharedString(s) => s,
        _ => continue,
      };

      if data.starts_with("Version") {
        version = Some(data.to_string());
      }

      if data.starts_with("Extracted") {
        extracted = Some(data.to_string());
      }

      if version.is_some() && extracted.is_some() {
        break;
      }
    }

    let mut version = version.unwrap();
    let mut extracted = extracted.unwrap();

    version.remove_matches("Version ");
    version.remove_matches("\"");
    let version = version.parse::<u32>().unwrap();
    let version = Version::try_from(version)?;

    extracted.remove_matches("Extracted: ");
    let my_format = format_description!("[day]/[month]/[year]");
    let extracted = Date::parse(&extracted, &my_format)?;

    Ok(Metadata { version, extracted })
  }
}
