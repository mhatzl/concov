//! Types for the loose Cobertura [XML schema](https://github.com/cobertura/cobertura/blob/master/cobertura/src/site/htdocs/xml/coverage-loose.dtd).

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename = "coverage")]
pub struct Coverage {
    #[serde(rename = "@line-rate")]
    pub line_rate: Option<f64>,
    #[serde(rename = "@branch-rate")]
    pub branch_rate: Option<f64>,
    #[serde(rename = "@lines-covered")]
    pub lines_covered: Option<u64>,
    #[serde(rename = "@lines-valid")]
    pub lines_valid: Option<u64>,
    #[serde(rename = "@branches-covered")]
    pub branches_covered: Option<u64>,
    #[serde(rename = "@branches-valid")]
    pub branches_valid: Option<u64>,
    #[serde(rename = "@complexity")]
    pub complexity: Option<f64>,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "@timestamp")]
    pub timestamp: String,
    pub sources: Option<Sources>,
    pub packages: Packages,
}

impl std::fmt::Display for Coverage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
        writeln!(f, "<!DOCTYPE coverage SYSTEM \"https://github.com/cobertura/cobertura/blob/master/cobertura/src/site/htdocs/xml/coverage-loose.dtd\">")?;
        writeln!(f, "{}", quick_xml::se::to_string(self).unwrap_or_default())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Sources {
    pub source: Vec<String>,
}

impl std::ops::Deref for Sources {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl std::ops::DerefMut for Sources {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.source
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Packages {
    pub package: Vec<Package>,
}

impl std::ops::Deref for Packages {
    type Target = Vec<Package>;

    fn deref(&self) -> &Self::Target {
        &self.package
    }
}

impl std::ops::DerefMut for Packages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.package
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@line-rate")]
    pub line_rate: Option<f64>,
    #[serde(rename = "@branch-rate")]
    pub branch_rate: Option<f64>,
    #[serde(rename = "@complexity")]
    pub complexity: Option<f64>,
    pub classes: Classes,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Classes {
    pub class: Vec<Class>,
}

impl std::ops::Deref for Classes {
    type Target = Vec<Class>;

    fn deref(&self) -> &Self::Target {
        &self.class
    }
}

impl std::ops::DerefMut for Classes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.class
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Class {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@filename")]
    pub filename: PathBuf,
    #[serde(rename = "@line-rate")]
    pub line_rate: Option<f64>,
    #[serde(rename = "@branch-rate")]
    pub branch_rate: Option<f64>,
    #[serde(rename = "@complexity")]
    pub complexity: Option<f64>,
    pub methods: Methods,
    pub lines: Lines,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Methods {
    pub method: Vec<Method>,
}

impl std::ops::Deref for Methods {
    type Target = Vec<Method>;

    fn deref(&self) -> &Self::Target {
        &self.method
    }
}

impl std::ops::DerefMut for Methods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.method
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Method {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@signature")]
    pub signature: String,
    #[serde(rename = "@line-rate")]
    pub line_rate: Option<f64>,
    #[serde(rename = "@branch-rate")]
    pub branch_rate: Option<f64>,
    #[serde(rename = "@complexity")]
    pub complexity: Option<f64>,
    pub lines: Lines,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Lines {
    pub line: Vec<Line>,
}

impl std::ops::Deref for Lines {
    type Target = Vec<Line>;

    fn deref(&self) -> &Self::Target {
        &self.line
    }
}

impl std::ops::DerefMut for Lines {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.line
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Line {
    #[serde(rename = "@number")]
    pub number: u64,
    #[serde(rename = "@hits")]
    pub hits: Option<u64>,
    #[serde(rename = "@branch", default)]
    pub branch: bool,
    #[serde(
        rename = "@condition-coverage",
        skip_serializing_if = "Option::is_none"
    )]
    pub condition_coverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Conditions {
    pub condition: Vec<Condition>,
}

impl std::ops::Deref for Conditions {
    type Target = Vec<Condition>;

    fn deref(&self) -> &Self::Target {
        &self.condition
    }
}

impl std::ops::DerefMut for Conditions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.condition
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "@number")]
    pub number: u64,
    #[serde(rename = "@type")]
    pub condition_type: String,
    #[serde(rename = "@coverage")]
    pub coverage: String,
}
