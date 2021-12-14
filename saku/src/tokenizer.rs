use crate::SentenceTokenizerBuilder;

#[derive(Clone, Debug)]
pub struct SentenceTokenizer {
    pub(crate) eos: char,
    pub(crate) left_patterns: Vec<char>,
    pub(crate) right_patterns: Vec<char>,
    pub(crate) preserve_newline: bool,
}

impl Default for SentenceTokenizer {
    fn default() -> Self {
        SentenceTokenizerBuilder::new().build()
    }
}

impl SentenceTokenizer {
    const STRING_CAPACITY: usize = 256;

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
    pub fn tokenize<'a>(&self, document: &'a str) -> Vec<&'a str> {
        let mut start: usize = 0;
        let mut sentences: Vec<&str> = Vec::new();
        let mut flags: Vec<bool> = vec![false; self.left_patterns.len()];
        let eos_size = self.eos.len_utf8();

        for (i, ch) in document.char_indices() {
            if (ch == '\n') | (ch == '\r') {
                if self.preserve_newline {
                    sentences.push(&document[start..i - 1]);
                    start = i + 1;
                }
                continue;
            }

            let in_parens = self.switch_flags_retun_in_parens(&ch, &mut flags);
            if in_parens {
                continue;
            }

            // During not in parens, simply we check whether the character is eos or not.
            if ch == self.eos {
                sentences.push(&document[start..i + eos_size]);
                start = i + eos_size;
            }
        }
        if start != document.len() {
            sentences.push(&document[start..document.len()]);
        }

        sentences
    }
}
