use std::io::Cursor;

use calamine::DataRef;
use time::{Date, macros::format_description};

use crate::Version;

#[derive(Debug, Clone)]
pub struct Metadata {
  pub version: Version,
  pub extracted: Date,
  pub rows: u32,
}

impl TryFrom<calamine::Xlsx<Cursor<Vec<u8>>>> for Metadata {
  type Error = crate::Error;

  fn try_from(mut value: calamine::Xlsx<Cursor<Vec<u8>>>) -> Result<Self, Self::Error> {
    let mut sheet = value.worksheet_cells_reader("README")?;

    let mut version = None;
    let mut extracted = None;
    let mut rows = None;

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

      if data.starts_with("Table") {
        rows = Some(data.to_string());
      }

      if version.is_some() && extracted.is_some() && rows.is_some() {
        break;
      }
    }

    let version = match version {
      Some(mut version) => {
        version.remove_matches("Version ");
        version.remove_matches("\"");
        let version = version.parse::<u32>().unwrap();
        Version::try_from(version)?
      },
      None => return Err(crate::Error::EmptyVersion),
    };

    let extracted = match extracted {
      Some(mut extracted) => {
        extracted.remove_matches("Extracted: ");
        let my_format = format_description!("[day]/[month]/[year]");
        Date::parse(&extracted, &my_format)?
      },
      None => return Err(crate::Error::EmptyExtractedDate),
    };

    let rows = match rows {
      Some(rows) => match rows.split_once("rows and") {
        Some((rows, _)) => {
          let mut rows = rows.to_string();
          rows.remove_matches("Table: ");
          rows.remove_matches(",");
          rows.trim().parse().unwrap()
        },
        None => return Err(crate::Error::InvalidRowCount(rows.clone()))?,
      },
      None => return Err(crate::Error::EmptyRowCount),
    };

    Ok(Metadata {
      version,
      extracted,
      rows,
    })
  }
}
