use crate::{CharTable, ControlFlow, SentenceTokenizer};

const DEFAULT_EOS: char = '。';
const DEFAULT_LEFT_PATTERNS: [char; 3] = ['（', '「', '『'];
const DEFAULT_RIGHT_PATTERNS: [char; 3] = ['）', '」', '』'];

#[derive(Clone, Debug)]
pub struct SentenceTokenizerBuilder {
    pub(crate) eos: char,
    pub(crate) left_patterns: Vec<char>,
    pub(crate) right_patterns: Vec<char>,
}

impl SentenceTokenizerBuilder {
    #[inline(always)]
    pub fn new() -> SentenceTokenizerBuilder {
        SentenceTokenizerBuilder {
            eos: DEFAULT_EOS,
            left_patterns: DEFAULT_LEFT_PATTERNS.to_vec(),
            right_patterns: DEFAULT_RIGHT_PATTERNS.to_vec(),
        }
    }
}

impl SentenceTokenizerBuilder {
    #[inline(always)]
    pub fn eos(self, eos: char) -> SentenceTokenizerBuilder {
        SentenceTokenizerBuilder {
            eos,
            left_patterns: self.left_patterns,
            right_patterns: self.right_patterns,
        }
    }

    #[inline(always)]
    pub fn patterns(self, patterns: &[[char; 2]]) -> SentenceTokenizerBuilder {
        let left_patterns: Vec<char> = patterns.iter().map(|p| p[0]).collect();
        let right_patterns: Vec<char> = patterns.iter().map(|p| p[1]).collect();
        SentenceTokenizerBuilder {
            eos: self.eos,
            left_patterns,
            right_patterns,
        }
    }

    #[inline(always)]
    pub fn build(&self) -> SentenceTokenizer {
        let eos = self.eos;
        let num_parens: u8 = self.left_patterns.len() as u8;

        let mut char_table: CharTable = CharTable::default();
        char_table.insert(eos, ControlFlow::Eos);
        for (flag_id, &l) in self.left_patterns.iter().enumerate() {
            char_table.insert(l, ControlFlow::LeftParens(flag_id as u8));
        }
        for (flag_id, &r) in self.right_patterns.iter().enumerate() {
            char_table.insert(r, ControlFlow::RightParens(flag_id as u8));
        }
        char_table.insert('\n', ControlFlow::LineBreaks);
        char_table.insert('\r', ControlFlow::LineBreaks);
        let eos_size = eos.len_utf8();

        SentenceTokenizer {
            eos,
            eos_size,
            num_parens,
            char_table,
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
