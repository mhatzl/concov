use std::{ffi::OsStr, path::PathBuf};

use crate::format::CoverageFormat;

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser)]
pub struct CliConfig {
    #[arg(long)]
    pub in_fmt: CoverageFormat,
    pub in_filepath: PathBuf,
    #[arg(long)]
    pub out_fmt: CoverageFormat,
    pub out_filepath: PathBuf,
}

impl TryFrom<CliConfig> for ConversionConfig {
    type Error = Box<dyn core::error::Error>;

    fn try_from(value: CliConfig) -> Result<Self, Self::Error> {
        let in_content = std::fs::read_to_string(&value.in_filepath)?;

        let in_data_fmt = DataFormat::try_from(value.in_filepath.extension())?;
        let out_data_fmt = DataFormat::try_from(value.out_filepath.extension())?;

        Ok(Self {
            in_fmt: value.in_fmt,
            in_content,
            in_data_fmt,
            out_fmt: value.out_fmt,
            out_data_fmt,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionConfig {
    pub in_fmt: CoverageFormat,
    pub in_content: String,
    pub in_data_fmt: DataFormat,
    pub out_fmt: CoverageFormat,
    pub out_data_fmt: DataFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    Json,
    Xml,
}

impl TryFrom<Option<&OsStr>> for DataFormat {
    type Error = Box<dyn core::error::Error>;

    fn try_from(value: Option<&OsStr>) -> Result<Self, Self::Error> {
        let value = match value {
            Some(s) => s.to_string_lossy(),
            None => return Err(Box::new(Error::UnsupportedExtension)),
        };

        Self::try_from(value.as_ref())
    }
}

impl TryFrom<&str> for DataFormat {
    type Error = Box<dyn core::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "xml" => Ok(Self::Xml),
            _ => Err(Box::new(Error::UnsupportedExtension)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    UnsupportedExtension,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unsupported file extension. Only JSON and XML are currently supported."
        )
    }
}

impl core::error::Error for Error {}
