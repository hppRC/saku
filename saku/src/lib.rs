mod builder;
mod char_table;
mod tokenizer;

#[derive(Clone, Debug)]
pub(crate) enum ControlFlow {
    Eos,
    LineBreaks,
    RightParens(u8),
    LeftParens(u8),
}
pub(crate) use char_table::CharTable;

pub use builder::SentenceTokenizerBuilder;
pub use tokenizer::SentenceTokenizer;
