use rustc_hash::FxHashSet;

use crate::SentenceTokenizer;

const DEFAULT_EOS: char = '。';
const DEFAULT_LEFT_PATTERNS: [char; 3] = ['（', '「', '『'];
const DEFAULT_RIGHT_PATTERNS: [char; 3] = ['）', '」', '』'];
const DEFAULT_CH_SET_WITHOUT_EOS: [char; 8] = ['（', '「', '『', '）', '」', '』', '\n', '\r'];
const DEFAULT_CH_SET: [char; 9] = ['。', '（', '「', '『', '）', '」', '』', '\n', '\r'];

#[derive(Clone, Debug)]
pub struct SentenceTokenizerBuilder<EOSType, PatternsType> {
    pub(crate) eos: EOSType,
    pub(crate) left_patterns: PatternsType,
    pub(crate) right_patterns: PatternsType,
}

impl SentenceTokenizerBuilder<(), ()> {
    pub fn new() -> Self {
        SentenceTokenizerBuilder {
            eos: (),
            left_patterns: (),
            right_patterns: (),
        }
    }
    #[inline]
    pub fn build(&self) -> SentenceTokenizer {
        let ch_set: FxHashSet<char> = FxHashSet::from_iter(DEFAULT_CH_SET);
        SentenceTokenizer {
            eos: DEFAULT_EOS,
            left_patterns: DEFAULT_LEFT_PATTERNS.to_vec(),
            right_patterns: DEFAULT_RIGHT_PATTERNS.to_vec(),
            ch_set,
        }
    }
}

impl<PatternsType> SentenceTokenizerBuilder<(), PatternsType> {
    pub fn eos(self, eos: char) -> SentenceTokenizerBuilder<char, PatternsType> {
        SentenceTokenizerBuilder {
            eos,
            left_patterns: self.left_patterns,
            right_patterns: self.right_patterns,
        }
    }
}

impl<EOSType> SentenceTokenizerBuilder<EOSType, ()> {
    pub fn patterns(self, patterns: &[[char; 2]]) -> SentenceTokenizerBuilder<EOSType, Vec<char>> {
        let left_patterns: Vec<char> = patterns.iter().map(|p| p[0]).collect();
        let right_patterns: Vec<char> = patterns.iter().map(|p| p[1]).collect();
        SentenceTokenizerBuilder {
            eos: self.eos,
            left_patterns,
            right_patterns,
        }
    }
}

impl SentenceTokenizerBuilder<char, ()> {
    pub fn build(&self) -> SentenceTokenizer {
        let mut ch_set = FxHashSet::from_iter(DEFAULT_CH_SET_WITHOUT_EOS);
        ch_set.insert(self.eos);
        SentenceTokenizer {
            eos: self.eos,
            left_patterns: DEFAULT_LEFT_PATTERNS.to_vec(),
            right_patterns: DEFAULT_RIGHT_PATTERNS.to_vec(),
            ch_set,
        }
    }
}
impl SentenceTokenizerBuilder<(), Vec<char>> {
    pub fn build(&self) -> SentenceTokenizer {
        let left_patterns = self.left_patterns.clone();
        let right_patterns = self.right_patterns.clone();
        let flat_patterns: Vec<char> = left_patterns.clone().into_iter().chain(right_patterns.clone().into_iter()).collect();
        let remainder = [DEFAULT_EOS, '\n', '\r'];
        let ch_set: FxHashSet<char> = FxHashSet::from_iter(flat_patterns.into_iter().chain(remainder));
        SentenceTokenizer {
            eos: DEFAULT_EOS,
            left_patterns,
            right_patterns,
            ch_set
        }
    }
}
impl SentenceTokenizerBuilder<char, Vec<char>> {
    pub fn build(&self) -> SentenceTokenizer {
        let eos = self.eos;
        let left_patterns = self.left_patterns.clone();
        let right_patterns = self.right_patterns.clone();
        let flat_patterns: Vec<char> = left_patterns.clone().into_iter().chain(right_patterns.clone().into_iter()).collect();
        let remainder = [eos, '\n', '\r'];
        let ch_set: FxHashSet<char> = FxHashSet::from_iter(flat_patterns.into_iter().chain(remainder));
        SentenceTokenizer {
            eos,
            left_patterns,
            right_patterns,
            ch_set,
        }
    }
}

impl<EOSType, PatternsType> SentenceTokenizerBuilder<EOSType, PatternsType> {
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
