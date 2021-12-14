use crate::SentenceTokenizer;

const DEFAULT_EOS: char = '。';
const DEFAULT_LEFT_PATTERNS: [char; 3] = ['（', '「', '『'];
const DEFAULT_RIGHT_PATTERNS: [char; 3] = ['）', '」', '』'];

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
        SentenceTokenizer {
            eos: DEFAULT_EOS,
            left_patterns: DEFAULT_LEFT_PATTERNS.to_vec(),
            right_patterns: DEFAULT_RIGHT_PATTERNS.to_vec(),
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
        SentenceTokenizer {
            eos: self.eos,
            left_patterns: DEFAULT_LEFT_PATTERNS.to_vec(),
            right_patterns: DEFAULT_RIGHT_PATTERNS.to_vec(),
        }
    }
}
impl SentenceTokenizerBuilder<(), Vec<char>> {
    pub fn build(&self) -> SentenceTokenizer {
        SentenceTokenizer {
            eos: DEFAULT_EOS,
            left_patterns: self.left_patterns.clone(),
            right_patterns: self.right_patterns.clone(),
        }
    }
}
impl SentenceTokenizerBuilder<char, Vec<char>> {
    pub fn build(&self) -> SentenceTokenizer {
        SentenceTokenizer {
            eos: self.eos,
            left_patterns: self.left_patterns.clone(),
            right_patterns: self.right_patterns.clone(),
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
