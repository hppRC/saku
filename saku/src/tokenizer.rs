use crate::SentenceTokenizerBuilder;

#[derive(Clone, Debug)]
pub struct SentenceTokenizer {
    pub(crate) eos: char,
    pub(crate) left_patterns: Vec<char>,
    pub(crate) right_patterns: Vec<char>,
}

impl Default for SentenceTokenizer {
    fn default() -> Self {
        SentenceTokenizerBuilder::new().build()
    }
}

impl SentenceTokenizer {
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

    #[inline]
    pub fn tokenize(&self, document: &str) -> Vec<String> {
        self.tokenize_remove_line_breaks(document)
    }

    #[inline]
    pub fn tokenize_remove_line_breaks(&self, document: &str) -> Vec<String> {
        let document = document.trim();
        let mut start: usize = 0;
        let mut sentences: Vec<String> = Vec::new();
        let mut sentence: String = String::new();
        let mut flags: Vec<bool> = vec![false; self.left_patterns.len()];
        let eos_size = self.eos.len_utf8();

        for (i, ch) in document.char_indices() {
            if (ch == '\n') || (ch == '\r') {
                sentence.push_str(&document[start..i]);
                start = i + 1;
                continue;
            }

            // CAUTION: This function call have a side effect in order to improve performance
            // by switching flags and returning whether we are in parens or not simultaneously.
            // Each flag of `flags` representing can be changed.
            let in_parens = self.switch_flags_retun_in_parens(&ch, &mut flags);
            if in_parens {
                continue;
            }

            if ch == self.eos {
                sentence.push_str(&document[start..i + eos_size]);
                sentences.push(sentence);
                sentence = String::new();
                start = i + eos_size;
            }
        }
        if start < document.len() {
            sentence.push_str(&document[start..document.len()]);
            sentences.push(sentence);
        }

        sentences
    }

    // This function returns a vector of references of sentences, considering line breaks as the terminator.
    // `tokenize` and `tokenize_raw` have a different return type to improve performance.
    #[inline]
    pub fn tokenize_raw<'a>(&self, document: &'a str) -> Vec<&'a str> {
        let document = document.trim();
        let mut start: usize = 0;
        let mut sentences: Vec<&'a str> = Vec::new();
        let mut flags: Vec<bool> = vec![false; self.left_patterns.len()];
        let eos_size = self.eos.len_utf8();

        for (i, ch) in document.char_indices() {
            if (ch == '\n') || (ch == '\r') {
                if i != start {
                    sentences.push(&document[start..i]);
                }
                start = i + 1;                    
                continue;
            }

            // CAUTION: This function call have a side effect in order to improve performance
            // by switching flags and returning whether we are in parens or not simultaneously.
            // Each flag of `flags` representing can be changed.
            let in_parens = self.switch_flags_retun_in_parens(&ch, &mut flags);
            if in_parens {
                continue;
            }

            if ch == self.eos {
                sentences.push(&document[start..i + eos_size]);
                start = i + eos_size;
            }
        }
        if start < document.len() {
            sentences.push(&document[start..document.len()]);
        }

        sentences
    }
}
