use pyo3::prelude::*;
use regex::{Captures, Regex};

#[pyclass]
pub struct SentenceTokenizer {
    period: String,
    alt_period: String,
    period_to: String,
    pattern: Regex,
}

#[pymethods]
impl SentenceTokenizer {
    const DEFAULT_PERIOD: &'static str = "。";
    const DEFAULT_ALT_PERIOD: &'static str = "__PERIOD__";
    const DEFAULT_PATTERN_STR: &'static str = r"（.*?）|「.*?」|『.*?』";

    #[new]
    fn __new__(
        period: Option<String>,
        pattern: Option<String>,
        patterns: Option<Vec<String>>,
        alt_period: Option<String>,
    ) -> PyResult<Self> {
        let period = period.unwrap_or(Self::DEFAULT_PERIOD.to_string());
        let period_to = format!("{}\n", &period);
        let pattern = pattern.map(|p| Regex::new(&p).unwrap()).unwrap_or({
            patterns
                .map(|patterns| Regex::new(&patterns.join("|")).unwrap())
                .unwrap_or(Regex::new(Self::DEFAULT_PATTERN_STR).unwrap())
        });
        let alt_period = alt_period.unwrap_or(Self::DEFAULT_ALT_PERIOD.to_string());
        Ok(Self {
            period,
            period_to,
            pattern,
            alt_period,
        })
    }

    fn tokenize_text(&self, text: &str) -> PyResult<Vec<String>> {
        let text: String = self
            .pattern
            .replace_all(text, |caps: &Captures| {
                caps[0].replace(&self.period, &self.alt_period)
            })
            .into();
        let text = text.replace(&self.period, &self.period_to);
        let sentences: Vec<String> = text
            .trim()
            .split("\n")
            .map(|s| s.replace(&self.alt_period, &self.period))
            .collect();
        Ok(sentences)
    }

    fn tokenize(&self, document: &str) -> PyResult<Vec<String>> {
        Ok(document
            .split("\n")
            .map(|text| self.tokenize_text(text).unwrap())
            .flatten()
            .collect())
    }
}

#[pymodule]
fn saku(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SentenceTokenizer>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
