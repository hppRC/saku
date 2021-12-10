#[derive(Clone, Debug)]
pub struct SentenceTokenizer {
    eos: char,
    left_patterns: Vec<char>,
    right_patterns: Vec<char>,
    preserve_newline: bool,
}

const DEFAULT_EOS: char = '。';
const DEFAULT_PATTERNS: [[char; 2]; 3] = [['（', '）'], ['「', '」'], ['『', '』']];
const DEFAULT_LEFT_PATTERNS: [char; 3] = ['（', '「', '『'];
const DEFAULT_RIGHT_PATTERNS: [char; 3] = ['）', '」', '』'];

impl Default for SentenceTokenizer {
    fn default() -> Self {
        Self {
            eos: DEFAULT_EOS,
            left_patterns: DEFAULT_LEFT_PATTERNS.to_vec(),
            right_patterns: DEFAULT_RIGHT_PATTERNS.to_vec(),
            preserve_newline: false,
        }
    }
}

#[derive(Clone, Debug)]
struct SentenceTokenizerBuilder<EOSType, PatternsType, PreserveNewlineType> {
    eos: Option<EOSType>,
    left_patterns: Option<PatternsType>,
    right_patterns: Option<PatternsType>,
    preserve_newline: Option<PreserveNewlineType>,
}

impl SentenceTokenizerBuilder<(), (), ()> {
    pub fn new() -> Self {
        SentenceTokenizerBuilder {
            eos: None,
            left_patterns: None,
            right_patterns: None,
            preserve_newline: None,
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
            eos: Some(eos),
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
            left_patterns: Some(left_patterns),
            right_patterns: Some(right_patterns),
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
            preserve_newline: Some(preserve_newline),
        }
    }
}

impl<EOSType, PatternsType, PreserveNewlineType>
    SentenceTokenizerBuilder<EOSType, PatternsType, PreserveNewlineType>
where
    EOSType: Into<char>,
    PatternsType: Into<Vec<char>>,
    PreserveNewlineType: Into<bool>,
{
    pub fn build(self) -> SentenceTokenizer {
        let eos: char = self.eos.map(Into::into).unwrap_or(DEFAULT_EOS);
        let left_patterns: Vec<char> = self
            .left_patterns
            .map(Into::into)
            .unwrap_or(DEFAULT_LEFT_PATTERNS.to_vec());
        let right_patterns: Vec<char> = self
            .right_patterns
            .map(Into::into)
            .unwrap_or(DEFAULT_RIGHT_PATTERNS.to_vec());
        let preserve_newline: bool = self.preserve_newline.map(Into::into).unwrap_or(false);

        SentenceTokenizer {
            eos,
            left_patterns,
            right_patterns,
            preserve_newline,
        }
    }
}

impl SentenceTokenizer {
    const DEFAULT_EOS: char = '。';
    const DEFAULT_PATTERNS: [[char; 2]; 3] = [['（', '）'], ['「', '」'], ['『', '』']];
    const SENTENCES_CAPACITY: usize = 1024;
    const STRING_CAPACITY: usize = 256;

    pub fn new(eos: Option<char>, patterns: Option<&[[char; 2]]>, preserve_newline: bool) -> Self {
        let eos: char = eos.unwrap_or(Self::DEFAULT_EOS);
        let patterns: Vec<[char; 2]> = patterns.unwrap_or(&Self::DEFAULT_PATTERNS).to_vec();
        let left_patterns: Vec<char> = patterns.iter().map(|p| p[0]).collect();
        let right_patterns: Vec<char> = patterns.iter().map(|p| p[1]).collect();

        Self {
            eos,
            left_patterns,
            right_patterns,
            preserve_newline,
        }
    }

    #[inline]
    fn switch_flags_retun_in_parens(&self, ch: &char, flags: &mut Vec<bool>) -> bool {
        for (l, f) in self.left_patterns.iter().zip(flags.iter_mut()) {
            if ch == l {
                *f = true;
                return true;
            }
        }
        let mut ret = false;
        for (r, f) in self.right_patterns.iter().zip(flags.iter_mut()) {
            *f &= !(ch == r);
            ret |= *f;
        }
        ret
    }

    // copy if `document` is a reference (&str)
    // move if `document` have an ownership (String)
    #[inline]
    pub fn tokenize(&self, document: impl Into<String>) -> Vec<String> {
        let document: String = document.into();
        let mut flags: Vec<bool> = vec![false; self.left_patterns.len()];
        let mut sentences: Vec<String> =
            Vec::with_capacity(document.len() / Self::SENTENCES_CAPACITY);
        let mut current_sentence: String = String::with_capacity(Self::STRING_CAPACITY);

        for ch in document.chars() {
            if (ch == '\n') | (ch == '\r') {
                if self.preserve_newline {
                    sentences.push(current_sentence);
                    current_sentence = String::with_capacity(Self::STRING_CAPACITY);
                }
                continue;
            }

            let in_parens = self.switch_flags_retun_in_parens(&ch, &mut flags);
            if in_parens {
                current_sentence.push(ch);
                continue;
            }

            // During not in parens, simply we check whether the character is eos or not.
            if ch == self.eos {
                current_sentence.push(ch);
                sentences.push(current_sentence);
                current_sentence = String::with_capacity(Self::STRING_CAPACITY);
            } else {
                current_sentence.push(ch);
            }
        }
        if !current_sentence.is_empty() {
            sentences.push(current_sentence);
        }

        sentences
    }
}
