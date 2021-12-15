use rustc_hash::FxHashMap;
use std::borrow::Cow;

use crate::SentenceTokenizerBuilder;

#[derive(Clone, Debug)]
pub(crate) enum ControlFlow {
    Eos,
    LineBreaks,
    RightParens(usize),
    LeftParens(usize),
}

#[derive(Clone, Debug)]
pub struct SentenceTokenizer {
    pub(crate) eos: char,
    pub(crate) num_parens: usize,
    pub(crate) chmap: FxHashMap<char, ControlFlow>,
}

impl Default for SentenceTokenizer {
    fn default() -> Self {
        SentenceTokenizerBuilder::new().build()
    }
}

impl SentenceTokenizer {
    #[inline]
    fn char_to_control_flow(&self, ch: &char) -> Option<&ControlFlow> {
        self.chmap.get(ch)
    }

    #[inline]
    pub fn tokenize<'a>(&self, document: &'a str) -> Vec<Cow<'a, str>> {
        self.tokenize_ignore_line_breaks(document)
    }

    #[inline(always)]
    pub fn process_line_breaks<'a>(&self, sentence: &mut String, start: &mut usize, i: usize, document: &'a str) {
        sentence.push_str(&document[*start..i]);
        *start = i + 1;
    }

    #[inline(always)]
    pub fn process_left_parens<'a>(&self, flags: &mut Vec<usize>, nest_count: &mut usize, flag_id: usize) {
        flags[flag_id] += 1;
        *nest_count += 1;
    }

    #[inline(always)]
    pub fn process_right_parens<'a>(&self, flags: &mut Vec<usize>, nest_count: &mut usize, flag_id: usize) {
        if flags[flag_id] > 0 {
            flags[flag_id] -= 1;
            *nest_count -= 1;
        }
    }

    #[inline(always)]
    pub fn process_eos<'a>(&self, sentences: &mut Vec<Cow<'a, str>>, mut sentence: String, start: &mut usize, i: usize, nest_count: usize, eos_size: usize, document: &'a str) {
        if nest_count > 0 {
            return;
        }
        if sentence.is_empty() {
            sentences.push(Cow::Borrowed(&document[*start..i + eos_size]));
        } else {
            sentence.push_str(&document[*start..i + eos_size]);
            sentences.push(Cow::Owned(sentence));
        }
        *start = i + eos_size;
    }


    #[inline]
    pub fn tokenize_ignore_line_breaks<'a>(&self, document: &'a str) -> Vec<Cow<'a, str>> {
        let document: &'a str = document.trim();
        let mut start: usize = 0;
        let mut sentences: Vec<Cow<'a, str>> = Vec::new();
        let mut sentence: String = String::new();
        let mut flags: Vec<usize> = vec![0; self.num_parens];
        let mut nest_count: usize = 0;
        let eos_size = self.eos.len_utf8();

        for (i, flow) in document.char_indices().filter_map(|(i, ch)| {
            if let Some(flow) = self.char_to_control_flow(&ch) {
                Some((i, flow))
            } else {
                None
            }
        }) {
            match *flow {
                ControlFlow::LineBreaks => self.process_line_breaks(&mut sentence, &mut start, i, &document),
                ControlFlow::LeftParens(flag_id) => self.process_left_parens(&mut flags, &mut nest_count, flag_id),
                ControlFlow::RightParens(flag_id) => self.process_right_parens(&mut flags, &mut nest_count, flag_id),
                ControlFlow::Eos => {
                    self.process_eos(&mut sentences, sentence, &mut start, i, nest_count, eos_size, document);
                    sentence = String::new();
                }
                ,
            }
        }
        if start < document.len() {
            if sentence.is_empty() {
                sentences.push(Cow::Borrowed(&document[start..document.len()]));
            } else {
                sentence.push_str(&document[start..document.len()]);
                sentences.push(Cow::Owned(sentence));
            }
        }

        sentences
    }

    // This function returns a vector of references of sentences, considering line breaks as the terminator.
    // `tokenize` and `tokenize_raw` have a different return type to improve performance.
    #[inline]
    pub fn tokenize_raw<'a>(&self, document: &'a str) -> Vec<&'a str> {
        todo!();
        let document = document.trim();
        // let mut start: usize = 0;
        // let mut sentences: Vec<&'a str> = Vec::new();
        // let mut flags: Vec<bool> = vec![false; self.left_patterns.len()];
        // let eos_size = self.eos.len_utf8();

        // for (i, ch) in document
        //     .char_indices()
        //     .filter(|(_, ch)| self.ch_set.contains(ch))
        // {
        //     if (ch == '\n') || (ch == '\r') {
        //         if i != start {
        //             sentences.push(&document[start..i]);
        //         }
        //         start = i + 1;
        //         continue;
        //     }

        //     // CAUTION: This function call have a side effect in order to improve performance
        //     // by switching flags and returning whether we are in parens or not simultaneously.
        //     // Each flag of `flags` representing can be changed.
        //     let in_parens = self.switch_flags_retun_in_parens(&ch, &mut flags);
        //     if in_parens {
        //         continue;
        //     }

        //     if ch == self.eos {
        //         sentences.push(&document[start..i + eos_size]);
        //         start = i + eos_size;
        //     }
        // }
        // if start < document.len() {
        //     sentences.push(&document[start..document.len()]);
        // }

        // sentences
    }
}
