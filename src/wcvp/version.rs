#[derive(Debug, Clone, Copy)]
pub enum Version {
  V15,
}

impl TryFrom<u32> for Version {
  type Error = crate::Error;

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    match value {
      15 => Ok(Version::V15),
      v => Err(crate::Error::InvalidVersion(v)),
    }
  }
}

impl From<Version> for u32 {
  fn from(value: Version) -> Self {
    match value {
      Version::V15 => 15,
    }
  }
}
