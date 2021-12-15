use rustc_hash::FxHashSet;

use crate::SentenceTokenizer;

const DEFAULT_EOS: char = '。';
const DEFAULT_LEFT_PATTERNS: [char; 3] = ['（', '「', '『'];
const DEFAULT_RIGHT_PATTERNS: [char; 3] = ['）', '」', '』'];

#[derive(Clone, Debug)]
pub struct SentenceTokenizerBuilder {
    pub(crate) eos: char,
    pub(crate) left_patterns: Vec<char>,
    pub(crate) right_patterns: Vec<char>,
}

impl SentenceTokenizerBuilder {
    pub fn new() -> SentenceTokenizerBuilder {
        SentenceTokenizerBuilder {
            eos: DEFAULT_EOS,
            left_patterns: DEFAULT_LEFT_PATTERNS.to_vec(),
            right_patterns: DEFAULT_RIGHT_PATTERNS.to_vec(),
        }
    }
}

impl SentenceTokenizerBuilder {
    pub fn eos(self, eos: char) -> SentenceTokenizerBuilder {
        SentenceTokenizerBuilder {
            eos,
            left_patterns: self.left_patterns,
            right_patterns: self.right_patterns,
        }
    }

    pub fn patterns(self, patterns: &[[char; 2]]) -> SentenceTokenizerBuilder {
        let left_patterns: Vec<char> = patterns.iter().map(|p| p[0]).collect();
        let right_patterns: Vec<char> = patterns.iter().map(|p| p[1]).collect();
        SentenceTokenizerBuilder {
            eos: self.eos,
            left_patterns,
            right_patterns,
        }
    }

    fn make_flat_patterns(&self) -> Vec<char> {
        self.left_patterns
            .clone()
            .clone()
            .into_iter()
            .chain(self.right_patterns.clone().into_iter())
            .collect()
    }

    pub fn build(&self) -> SentenceTokenizer {
        let eos = self.eos;
        let flat_patterns: Vec<char> = self.make_flat_patterns();
        let remainder = [eos, '\n', '\r'];
        let ch_set: FxHashSet<char> =
            FxHashSet::from_iter(flat_patterns.into_iter().chain(remainder));
        SentenceTokenizer {
            eos,
            left_patterns: self.left_patterns.clone(),
            right_patterns: self.right_patterns.clone(),
            ch_set,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn it_works() {
        let tokenizer = SentenceTokenizerBuilder::new()
            .eos('。')
            .patterns(&[['（', '）'], ['「', '」']])
            .build();
        assert_eq!('。', tokenizer.eos);
    }
}
