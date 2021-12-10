// use rayon::prelude::*;

pub struct SentenceTokenizer {
    eos: char,
    left_patterns: Vec<char>,
    right_patterns: Vec<char>,
}

impl SentenceTokenizer {
    const DEFAULT_EOS: char = '。';
    const DEFAULT_PATTERNS: [[char; 2]; 3] = [['（', '）'], ['「', '」'], ['『', '』']];

    pub fn new(eos: Option<char>, patterns: Option<&[[char; 2]]>) -> Self {
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
    fn switch_flags_retun_in_parens(&self, ch: &char, flags: &mut Vec<bool>) -> bool {
        for (l, f) in self.left_patterns.iter().zip(flags.iter_mut()) {
            if ch == l {
                *f = true;
                return true;
            }
        }
        let mut ret = false;
        for (r, f) in self.right_patterns.iter().zip(flags.iter_mut()) {
            if ch == r {
                *f = false;
            }
            ret |= *f;
        };
        ret
    }

    #[inline]
    fn is_newline_char(&self, ch: &char) -> bool {
        (*ch == '\n') | (*ch == '\r')
    }

    // copy if `document` is a reference (&str)
    // move if `document` have an ownership (String)
    #[inline]
    pub fn tokenize(&self, document: impl Into<String>, preserve_newline: bool) -> Vec<String> {
        let document: String = document.into();
        let sentences_cap = 1024;
        let string_cap = 256;
        let mut flags: Vec<bool> = vec![false; self.left_patterns.len()];
        let mut sentences: Vec<String> = Vec::with_capacity(document.len() / sentences_cap);
        let mut current_sentence: String = String::with_capacity(string_cap);

        for ch in document.chars() {
            if self.is_newline_char(&ch) {
                if preserve_newline {
                    sentences.push(current_sentence);
                    current_sentence = String::with_capacity(string_cap);
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
                current_sentence = String::with_capacity(string_cap);
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
