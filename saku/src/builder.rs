use crate::SentenceTokenizer;

const DEFAULT_EOS: char = '。';
const DEFAULT_LEFT_PATTERNS: [char; 3] = ['（', '「', '『'];
const DEFAULT_RIGHT_PATTERNS: [char; 3] = ['）', '」', '』'];

#[derive(Clone, Debug)]
pub struct SentenceTokenizerBuilder<EOSType, PatternsType, PreserveNewlineType> {
    pub(crate) eos: EOSType,
    pub(crate) left_patterns: PatternsType,
    pub(crate) right_patterns: PatternsType,
    pub(crate) preserve_newline: PreserveNewlineType,
}

impl SentenceTokenizerBuilder<(), (), ()> {
    pub fn new() -> Self {
        SentenceTokenizerBuilder {
            eos: (),
            left_patterns: (),
            right_patterns: (),
            preserve_newline: (),
        }
    }
    #[inline]
    pub fn build(&self) -> SentenceTokenizer {
        SentenceTokenizer {
            eos: DEFAULT_EOS,
            left_patterns: DEFAULT_LEFT_PATTERNS.to_vec(),
            right_patterns: DEFAULT_RIGHT_PATTERNS.to_vec(),
            preserve_newline: false,
        }
    }
}

impl<PatternsType, PreserveNewlineType>
    SentenceTokenizerBuilder<(), PatternsType, PreserveNewlineType>
{
    pub fn eos(
        self,
        eos: char,
    ) -> SentenceTokenizerBuilder<char, PatternsType, PreserveNewlineType> {
        SentenceTokenizerBuilder {
            eos,
            left_patterns: self.left_patterns,
            right_patterns: self.right_patterns,
            preserve_newline: self.preserve_newline,
        }
    }
}

impl<EOSType, PreserveNewlineType> SentenceTokenizerBuilder<EOSType, (), PreserveNewlineType> {
    pub fn patterns(
        self,
        patterns: &[[char; 2]],
    ) -> SentenceTokenizerBuilder<EOSType, Vec<char>, PreserveNewlineType> {
        let left_patterns: Vec<char> = patterns.iter().map(|p| p[0]).collect();
        let right_patterns: Vec<char> = patterns.iter().map(|p| p[1]).collect();
        SentenceTokenizerBuilder {
            eos: self.eos,
            left_patterns,
            right_patterns,
            preserve_newline: self.preserve_newline,
        }
    }
}

impl<EOSType, PatternsType> SentenceTokenizerBuilder<EOSType, PatternsType, ()> {
    pub fn preserve_newline(
        self,
        preserve_newline: bool,
    ) -> SentenceTokenizerBuilder<EOSType, PatternsType, bool> {
        SentenceTokenizerBuilder {
            eos: self.eos,
            left_patterns: self.left_patterns,
            right_patterns: self.right_patterns,
            preserve_newline,
        }
    }
}

impl SentenceTokenizerBuilder<char, (), ()> {
    pub fn build(&self) -> SentenceTokenizer {
        SentenceTokenizer {
            eos: self.eos,
            left_patterns: DEFAULT_LEFT_PATTERNS.to_vec(),
            right_patterns: DEFAULT_RIGHT_PATTERNS.to_vec(),
            preserve_newline: false,
        }
    }
}
impl SentenceTokenizerBuilder<(), Vec<char>, ()> {
    pub fn build(&self) -> SentenceTokenizer {
        SentenceTokenizer {
            eos: DEFAULT_EOS,
            left_patterns: self.left_patterns.clone(),
            right_patterns: self.right_patterns.clone(),
            preserve_newline: false,
        }
    }
}
impl SentenceTokenizerBuilder<(), (), bool> {
    pub fn build(&self) -> SentenceTokenizer {
        SentenceTokenizer {
            eos: DEFAULT_EOS,
            left_patterns: DEFAULT_LEFT_PATTERNS.to_vec(),
            right_patterns: DEFAULT_RIGHT_PATTERNS.to_vec(),
            preserve_newline: self.preserve_newline,
        }
    }
}
impl SentenceTokenizerBuilder<char, Vec<char>, ()> {
    pub fn build(&self) -> SentenceTokenizer {
        SentenceTokenizer {
            eos: self.eos,
            left_patterns: self.left_patterns.clone(),
            right_patterns: self.right_patterns.clone(),
            preserve_newline: false,
        }
    }
}
impl SentenceTokenizerBuilder<(), Vec<char>, bool> {
    pub fn build(&self) -> SentenceTokenizer {
        SentenceTokenizer {
            eos: DEFAULT_EOS,
            left_patterns: self.left_patterns.clone(),
            right_patterns: self.right_patterns.clone(),
            preserve_newline: self.preserve_newline,
        }
    }
}
impl SentenceTokenizerBuilder<char, (), bool> {
    pub fn build(&self) -> SentenceTokenizer {
        SentenceTokenizer {
            eos: self.eos,
            left_patterns: DEFAULT_LEFT_PATTERNS.to_vec(),
            right_patterns: DEFAULT_RIGHT_PATTERNS.to_vec(),
            preserve_newline: self.preserve_newline,
        }
    }
}
impl SentenceTokenizerBuilder<char, Vec<char>, bool> {
    pub fn build(&self) -> SentenceTokenizer {
        SentenceTokenizer {
            eos: self.eos,
            left_patterns: self.left_patterns.clone(),
            right_patterns: self.right_patterns.clone(),
            preserve_newline: self.preserve_newline,
        }
    }
}

mod test {
    use crate::*;

    #[test]
    fn it_works() {
        let tokenizer = SentenceTokenizerBuilder::new()
            .eos('。')
            .preserve_newline(false)
            .build();
        assert_eq!('。', tokenizer.eos);
    }
}
