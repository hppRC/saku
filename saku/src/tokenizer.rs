use crate::{CharTable, ControlFlow, SentenceTokenizerBuilder};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct SentenceTokenizer {
    pub eos: char,
    pub(crate) eos_size: usize,
    pub(crate) num_parens: u8,
    pub(crate) char_table: CharTable,
}

impl Default for SentenceTokenizer {
    #[inline(always)]
    fn default() -> Self {
        SentenceTokenizerBuilder::new().build()
    }
}

impl SentenceTokenizer {
    #[inline(always)]
    fn char_to_control_flow(&self, ch: &char) -> Option<&ControlFlow> {
        // self.chmap.get(ch)
        self.char_table.get(*ch)
    }

    #[inline(always)]
    pub fn tokenize<'a>(&self, document: &'a str) -> Vec<Cow<'a, str>> {
        self.tokenize_ignore_line_breaks(document)
    }

    #[inline(always)]
    pub fn process_line_breaks<'a>(
        &self,
        sentence: &mut String,
        start: &mut usize,
        i: usize,
        document: &'a str,
    ) {
        sentence.push_str(&document[*start..i]);
        *start = i + 1;
    }

    #[inline(always)]
    pub fn process_left_parens<'a>(
        &self,
        flags: &mut Vec<u8>,
        nest_count: &mut u8,
        flag_id: usize,
    ) {
        flags[flag_id] += 1;
        *nest_count += 1;
    }

    #[inline(always)]
    pub fn process_right_parens<'a>(
        &self,
        flags: &mut Vec<u8>,
        nest_count: &mut u8,
        flag_id: usize,
    ) {
        if flags[flag_id] > 0 {
            flags[flag_id] -= 1;
            *nest_count -= 1;
        }
    }

    #[inline(always)]
    pub fn tokenize_ignore_line_breaks<'a>(&self, document: &'a str) -> Vec<Cow<'a, str>> {
        let document: &'a str = document.trim();
        let mut start: usize = 0;
        let mut sentences: Vec<Cow<'a, str>> = Vec::new();
        let mut sentence: String = String::new();
        let mut flags: Vec<u8> = vec![0; self.num_parens as usize];
        let mut nest_count: u8 = 0;

        for (i, ch) in document.char_indices() {
            if let Some(flow) = self.char_to_control_flow(&ch) {
                match *flow {
                    ControlFlow::LineBreaks => {
                        self.process_line_breaks(&mut sentence, &mut start, i, &document)
                    }
                    ControlFlow::LeftParens(flag_id) => {
                        self.process_left_parens(&mut flags, &mut nest_count, flag_id as usize)
                    }
                    ControlFlow::RightParens(flag_id) => {
                        self.process_right_parens(&mut flags, &mut nest_count, flag_id as usize)
                    }
                    ControlFlow::Eos => {
                        if nest_count > 0 {
                            continue;
                        }
                        if sentence.is_empty() {
                            sentences.push(Cow::Borrowed(&document[start..i + self.eos_size]));
                        } else {
                            sentence.push_str(&document[start..i + self.eos_size]);
                            sentences.push(Cow::Owned(sentence));
                            sentence = String::new();
                        }
                        start = i + self.eos_size;
                    }
                }
            }
        }
        if start < document.len() {
            if sentence.is_empty() {
                sentences.push(Cow::Borrowed(&document[start..]));
            } else {
                sentence.push_str(&document[start..]);
                sentences.push(Cow::Owned(sentence));
            }
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
        let mut flags: Vec<u8> = vec![0; self.num_parens as usize];
        let mut nest_count: u8 = 0;

        for (i, ch) in document.char_indices() {
            if let Some(flow) = self.char_to_control_flow(&ch) {
                match *flow {
                    ControlFlow::LineBreaks => {
                        if i != start {
                            sentences.push(&document[start..i]);
                        }
                        nest_count = 0;
                        start = i + 1;
                    }
                    ControlFlow::LeftParens(flag_id) => {
                        self.process_left_parens(&mut flags, &mut nest_count, flag_id as usize)
                    }
                    ControlFlow::RightParens(flag_id) => {
                        self.process_right_parens(&mut flags, &mut nest_count, flag_id as usize)
                    }
                    ControlFlow::Eos => {
                        if nest_count > 0 {
                            continue;
                        }
                        sentences.push(&document[start..i + self.eos_size]);
                        start = i + self.eos_size;
                    }
                }
            }
        }
        if start < document.len() {
            sentences.push(&document[start..]);
        }

        sentences
    }
}
