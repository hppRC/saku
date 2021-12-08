pub struct SentenceTokenizer {
    eos: char,
    left_patterns: Vec<char>,
    right_patterns: Vec<char>,
}

impl SentenceTokenizer {
    const DEFAULT_EOS: char = '。';
    const DEFAULT_PATTERNS: [[char; 2]; 3] = [['（', '）'], ['「', '」'], ['『', '』']];

    pub fn new(
        eos: Option<char>,
        patterns: Option<&[[char; 2]]>,
    ) -> Self {
        let eos: char = eos.unwrap_or(Self::DEFAULT_EOS);
        let patterns: Vec<[char; 2]> = patterns.unwrap_or(&Self::DEFAULT_PATTERNS).to_vec();
        let left_patterns: Vec<char> = patterns.iter().map(|p| p[0]).collect();
        let right_patterns: Vec<char> = patterns.iter().map(|p| p[1]).collect();

        Self {
            eos,
            left_patterns,
            right_patterns,
        }
    }

    #[inline]
    fn switch_flags(&self, ch: &char, flags: &mut Vec<bool>){
        for (i, l) in self.left_patterns.iter().enumerate() {
            if ch == l {
                flags[i] = true;
                return;
            }
        }
        for (i, r) in self.right_patterns.iter().enumerate() {
            if ch == r {
                flags[i] = false;
                return;
            }
        }
    }

    #[inline]
    fn is_newline_char(&self, ch: &char) -> bool {
        (*ch == '\n') | (*ch == '\r')
    }

    // copy if `document` is a reference (&str)
    // move if `document` have a ownership (String)
    #[inline]
    pub fn tokenize(
        &self,
        document: impl Into<String>,
        preserve_newline: bool,
    ) -> Vec<String> {
        let document: String = document.into();
        let cap = 128;
        let mut flags: Vec<bool> = vec![false; self.left_patterns.len()];
        let mut sentences: Vec<String> = vec![];
        let mut current_sentence: String = String::with_capacity(cap);
        
        for ch in document.chars() {
            if self.is_newline_char(&ch) {
                if preserve_newline {
                    sentences.push(current_sentence);
                    current_sentence = String::with_capacity(cap);
                }
                continue;
            }

            self.switch_flags(&ch, &mut flags);
            if flags.iter().any(|f| *f) {
                current_sentence.push(ch);
                continue;
            }

            if ch == self.eos {
                current_sentence.push(ch);
                sentences.push(current_sentence);
                current_sentence = String::with_capacity(cap);
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