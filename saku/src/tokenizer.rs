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
    const SENTENCES_CAPACITY: usize = 1024;
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
