use std::{
    fmt::{Display, Error, Formatter},
    ops::Range,
};

use chumsky::span::{Span, Spanned, WrappingSpan};

use crate::source::{SourceID, SourceStore};

pub type ByteOffset = usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceIDSpan {
    pub context: SourceID,
    pub start: ByteOffset,
    pub end: ByteOffset,
}

impl Span for SourceIDSpan {
    type Context = SourceID;
    type Offset = ByteOffset;

    fn new(context: Self::Context, range: Range<Self::Offset>) -> Self {
        SourceIDSpan {
            context,
            start: range.start,
            end: range.end,
        }
    }

    fn context(&self) -> Self::Context {
        self.context
    }

    fn start(&self) -> Self::Offset {
        self.start
    }

    fn end(&self) -> Self::Offset {
        self.end
    }
}

pub type SourceIDSpanned<T> = Spanned<T, SourceIDSpan>;

impl<T> WrappingSpan<T> for SourceIDSpan {
    type Spanned = SourceIDSpanned<T>;

    // Required methods
    fn make_wrapped(self, inner: T) -> Self::Spanned {
        SourceIDSpanned { inner, span: self }
    }
    fn inner_of(spanned: &Self::Spanned) -> &T {
        &spanned.inner
    }
    fn span_of(spanned: &Self::Spanned) -> &Self {
        &spanned.span
    }
}

pub struct SourceIDSpanWithStore<'src> {
    pub span: SourceIDSpan,
    pub sources: &'src SourceStore,
}

impl<'src> Display for SourceIDSpanWithStore<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let SourceIDSpanWithStore {
            span,
            sources,
        } = self;
        // let source_filename = sources
        //     .get_filename(*source_id)
        //     .unwrap_or_else(|| panic!("No source with source id {}", source_id));

        let source_location = sources.map_span(span);
        write!(f, "{:?}", source_location)
    }
}
