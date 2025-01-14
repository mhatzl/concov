#[derive(
    Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, serde::Serialize, serde::Deserialize,
)]
pub enum CoverageFormat {
    /// Cobertura v4 XML coverage format.
    ///
    /// Official schema is defined [here](https://github.com/cobertura/cobertura/blob/master/cobertura/src/site/htdocs/xml/coverage-04.dtd).
    CoberturaV4,
    /// Cobertura loose XML coverage format.
    ///
    /// Official schema is defined [here](https://github.com/cobertura/cobertura/blob/master/cobertura/src/site/htdocs/xml/coverage-loose.dtd).
    CoberturaLoose,
}
