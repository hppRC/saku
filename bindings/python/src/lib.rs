use std::borrow::Cow;

use pyo3::prelude::*;
use saku::{SentenceTokenizer, SentenceTokenizerBuilder};

#[pyclass(name = "SentenceTokenizer")]
#[pyo3(text_signature = "(self, eos, patterns)")]
#[derive(Clone, Debug)]
pub struct PySentenceTokenizer {
    tokenizer: SentenceTokenizer,
}

#[pymethods]
impl PySentenceTokenizer {
    #[inline(always)]
    #[new]
    fn __new__(eos: Option<String>, patterns: Option<Vec<String>>) -> Self {
        let mut tokenizer_builder = SentenceTokenizerBuilder::new();
        if let Some(eos) = eos {
            let ch = eos.chars().nth(0).unwrap();
            tokenizer_builder = tokenizer_builder.eos(ch);
        };
        if let Some(patterns_string) = patterns {
            let patterns: Vec<[char; 2]> = patterns_string
                .iter()
                .map(|pattern| {
                    let l = pattern.chars().nth(0).unwrap();
                    let r = pattern.chars().nth(1).unwrap();
                    [l, r]
                })
                .collect();
            tokenizer_builder = tokenizer_builder.patterns(&patterns);
        }
        let tokenizer = tokenizer_builder.build();
        Self { tokenizer }
    }

    #[inline(always)]
    pub fn tokenize<'a>(&self, document: &'a str) -> Vec<Cow<'a, str>> {
        self.tokenizer.tokenize(document)
    }

    #[inline(always)]
    pub fn tokenize_raw<'a>(&self, document: &'a str) -> Vec<&'a str> {
        self.tokenizer.tokenize_raw(document)
    }
}

#[pymodule]
fn saku(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySentenceTokenizer>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
