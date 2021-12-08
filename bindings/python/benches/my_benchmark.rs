use criterion::{criterion_group, criterion_main, Criterion};
use regex::{Captures, Regex};
use std::fs;
use anyhow::Result;

pub struct SentenceTokenizer {
    period: String,
    alt_period: String,
    period_newline: String,
    pattern: Regex,
}

impl SentenceTokenizer {
    const DEFAULT_PERIOD: &'static str = "。";
    const DEFAULT_ALT_PERIOD: &'static str = "__PERIOD__";
    const DEFAULT_PATTERN_STR: &'static str = r"（.*?）|「.*?」|『.*?』";

    fn new(
        period: Option<String>,
        pattern: Option<String>,
        patterns: Option<Vec<String>>,
        alt_period: Option<String>,
    ) -> Result<Self> {
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
            alt_period,
            period_newline,
            pattern,
        })
    }

    // #[inline]
    fn tokenize(
        &self,
        mut text: String,
        preserve_newline: Option<bool>,
    ) -> Result<Vec<String>> {
        let preserve_newline = preserve_newline.unwrap_or(false);
        // if !preserve_newline {
        //     text = Regex::new(&format!(r"\n|\r"))
        //         .unwrap()
        //         .replace_all(&text, "")
        //         .into_owned();
        // } // 35ms
        let a = self
            .pattern
            .replace_all(&text, |caps: &Captures| {
                caps[0].replace(&self.period, &self.alt_period)
            }) // 300ms
            .replace(&self.period, &self.period_newline) // 100ms
            .split("\n") // .collect::<Vec<_>>() 70ms
            .filter_map(|s| 
                if s.is_empty() {
                    None
                } else {
                    Some(s.replace(&self.alt_period, &self.period))
                }
            ) // .collect::<Vec<_>>() 100ms
            ;
        //     .collect())
        Ok(vec!["".into()])
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tokenize", |b| {
        let data_path = "../data/medium.txt";
        let tokenizer = SentenceTokenizer::new(None, None, None, None).unwrap();
        let text = fs::read_to_string(data_path).unwrap();
        b.iter(|| tokenizer.tokenize(text.clone(), None))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);