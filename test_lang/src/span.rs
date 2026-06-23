use chumsky::span::{SimpleSpan, Spanned};

pub type ByteOffset = usize;
pub type SourceID = usize;
pub type SourceIDSpan = SimpleSpan<ByteOffset, SourceID>;
pub type SourceIDSpanned<T> = Spanned<T, SourceIDSpan>;
