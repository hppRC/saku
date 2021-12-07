use pyo3::prelude::*;
use regex::{Captures, Regex};

#[pyclass]
pub struct SentenceTokenizer {
    period: String,
    alt_period: String,
    period_newline: String,
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
        let period_newline = format!("{}\n", &period);
        let pattern = pattern.map(|p| Regex::new(&p).unwrap()).unwrap_or({
            patterns
                .map(|patterns| Regex::new(&patterns.join("|")).unwrap())
                .unwrap_or(Regex::new(Self::DEFAULT_PATTERN_STR).unwrap())
        });
        let alt_period = alt_period.unwrap_or(Self::DEFAULT_ALT_PERIOD.to_string());
        Ok(Self {
            period,
            period_newline,
            pattern,
            alt_period,
        })
    }

    fn tokenize(&self, text: &str, preserve_newline: Option<bool>) -> PyResult<Vec<String>> {
        let preserve_newline = preserve_newline.unwrap_or(false);
        let text = if preserve_newline {
            text.to_string()
        } else {
            text.replace("\n", "").replace("\r", "")
        };
        Ok(self
            .pattern
            .replace_all(&text, |caps: &Captures| {
                caps[0].replace(&self.period, &self.alt_period)
            })
            .to_string()
            .replace(&self.period, &self.period_newline)
            .trim()
            .split("\n")
            .map(|s| s.replace(&self.alt_period, &self.period))
            .collect())
    }
}

#[pymodule]
fn saku(_: Python, m: &PyModule) -> PyResult<()> {
    println!("bbb");
    m.add_class::<SentenceTokenizer>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
