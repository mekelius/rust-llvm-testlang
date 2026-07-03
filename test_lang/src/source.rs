use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use logos::Logos;
use logos::skip;

use crate::helpers::LineCol;
use crate::span::ByteOffset;
use crate::span::SourceIDSpan;

pub type SourceID = usize;

// ****************************************** TYPES ***********************************************

/**
 * Stores the filename and contents of a source. Supports mapping byteoffsets to line col locations
 * */
#[derive(Debug, PartialEq)]
pub struct Source {
    pub filename: String,
    pub contents: String,
    linebreaks: Vec<usize>,
}

/**
 * Container for storing the sources and accessing them using ID:s.
 * Has convenience methods for mapping SourceIDSpans to SourceLocations.
 */
#[derive(Debug)]
pub struct SourceStore {
    sources: Vec<Source>,
}

pub const BUILTINS_SOURCE_ID: SourceID = 0;

/**
 * Location in a source. Contains a ref to the Source and thus cannot be stored long term.
 * For storage use SourceIDSpan instead.
 * Implements Display.
 */
#[derive(Debug, PartialEq)]
pub struct SourceLocation<'src> {
    pub source: &'src Source,
    pub line: usize,
    pub col: usize,
}

/**
 * A span in a source. Contains a ref to the Source and thus cannot be stored long term.
 * For storage use SourceIDSpan instead.
 * Implements Display.
 */
#[derive(Debug, PartialEq)]
pub struct SourceSpan<'src> {
    pub source: &'src Source,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

// ************************************** IPLEMENTATIONS ******************************************

// Maybe just run the regex instead...
#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
#[logos()]
pub enum NewlineLexer {
    #[regex(r"(?s)[^\n]+", skip, allow_greedy = true)]
    Skipped,

    #[token("\n")]
    Newline,
}

impl<'src> Source {
    pub fn new(filename: &str, contents: &str) -> Self {
        let linebreaks = NewlineLexer::lexer(contents)
            .spanned()
            .map(|(_, span)| span.start)
            .collect();

        Source {
            filename: filename.to_string(),
            contents: contents.to_string(),
            linebreaks,
        }
    }

    pub fn map_span(&'src self, span: &SourceIDSpan) -> (LineCol, LineCol) {
        (self.map_offset(span.start), self.map_offset(span.end))
    }

    // NOTE: Does not support utf8
    pub fn map_offset(&self, offset: ByteOffset) -> LineCol {
        if self.linebreaks.is_empty() {
            return LineCol {
                line: 1,
                col: offset + 1,
            };
        }

        let partition_point_index = self.linebreaks.partition_point(|x| *x < offset);

        if partition_point_index == 0 {
            return LineCol {
                line: 1,
                col: offset + 1,
            };
        }

        let last_linebreak_offset = self.linebreaks[partition_point_index - 1];

        let line = partition_point_index + 1;
        let col = offset - last_linebreak_offset;

        LineCol { line, col }
    }
}

impl<'src> SourceStore {
    pub fn new() -> SourceStore {
        let builtins_source = Source::new("!BUILTINS!", "");
        SourceStore { sources: vec![builtins_source] }
    }

    pub fn get_filename(&self, source_id: SourceID) -> Option<&String> {
        let source = self.sources.get(source_id)?;
        Some(&source.filename)
    }

    pub fn get_source(&self, source_id: SourceID) -> Option<&Source> {
        let source = self.sources.get(source_id)?;
        Some(&source)
    }

    pub fn add_source(&mut self, filename: &str, contents: &str) -> SourceID {
        self.sources.push(Source::new(filename, contents));
        self.sources.len() - 1
    }

    /** Maps a span (of byte offsets) into line and col numbers */
    pub fn map_offset(
        &'src self,
        source_id: SourceID,
        offset: ByteOffset,
    ) -> Option<SourceLocation<'src>> {
        let source = self.get_source(source_id)?;

        let LineCol { line, col } = source.map_offset(offset);
        Some(SourceLocation { source, line, col })
    }

    pub fn map_start(&'src self, span: &SourceIDSpan) -> SourceLocation<'src> {
        let SourceIDSpan {
            context: source_id,
            start,
            end: _,
        } = span;

        self.map_offset(*source_id, *start).unwrap_or_else(|| {
            panic!(
                "Encountered span with source_id {}, but no such source exists!",
                span.context
            )
        })
    }

    pub fn map_end(&'src self, span: &SourceIDSpan) -> SourceLocation<'src> {
        let SourceIDSpan {
            context: source_id,
            start: _,
            end,
        } = span;

        self.map_offset(*source_id, *end).unwrap_or_else(|| {
            panic!(
                "Encountered span with source_id {}, but no such source exists!",
                span.context
            )
        })
    }

    /** Maps a span (of byte offsets) into line and col numbers */
    pub fn map_span(&'src self, span: &SourceIDSpan) -> SourceSpan<'src> {
        let SourceIDSpan {
            context,
            start: _,
            end: _,
        } = span;
        let source = self.get_source(*context).unwrap_or_else(|| {
            panic!(
                "Encountered span with source_id {}, but no such source exists!",
                span.context
            )
        });

        let (
            LineCol {
                line: start_line,
                col: start_col,
            },
            LineCol {
                line: end_line,
                col: end_col,
            },
        ) = source.map_span(&span);
        SourceSpan {
            source,
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }
}

impl<'src> Display for SourceLocation<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}: {}:{}", self.source.filename, self.line, self.col)
    }
}

impl<'src> SourceSpan<'src> {
    pub fn get_start(&self) -> SourceLocation<'src> {
        SourceLocation {
            source: self.source,
            line: self.start_line,
            col: self.start_col,
        }
    }

    pub fn get_end(&self) -> SourceLocation<'src> {
        SourceLocation {
            source: self.source,
            line: self.end_line,
            col: self.end_col,
        }
    }
}

impl<'src> Display for SourceSpan<'src> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}: {}:{}-{}:{}",
            self.source.filename, self.start_line, self.start_col, self.end_line, self.end_col
        )
    }
}

// ****************************************** TESTS ***********************************************

#[cfg(test)]
mod tests {
    use chumsky::span::Span;
    use std::ops::Range;

    use super::*;

    mod source {
        use super::*;

        mod new {
            use super::*;

            #[test]
            fn gets_newlines_correctly() {
                let source = Source::new("testfile", "\n\n\nmain(){}\n\nthing(){}\n\n\n");
                assert_eq!(source.linebreaks, [0, 1, 2, 11, 12, 22, 23, 24]);
            }
        }

        mod map_offset {
            use super::*;

            #[test]
            fn calculates_line_and_col_correctly() {
                let source = Source::new("testfile1", "\n\n\nmain(){}\n\nthing(){}\n\n\n");
                assert_eq!(source.map_offset(15), LineCol { line: 6, col: 3 });

                let source = Source::new("testfile2", "\n\n\nmain(){}\n\n;\n;thing(){}\n\n\n");
                assert_eq!(source.map_offset(18), LineCol { line: 7, col: 4 });
            }

            #[test]
            fn handles_first_line() {
                let source = Source::new("testfile1", "01234567\n\nthing(){}\n\n\n");
                assert_eq!(source.map_offset(2), LineCol { line: 1, col: 3 });
            }

            #[test]
            fn handles_last_line() {
                let source = Source::new("testfile1", "012345\n\n89A\n\n\nEFG");
                assert_eq!(source.map_offset(15), LineCol { line: 6, col: 2 });
            }

            #[test]
            fn handles_offsets_past_end() {
                let source = Source::new("testfile1", "012345\n\n89A\n\n\nEFG");
                assert_eq!(source.map_offset(115), LineCol { line: 6, col: 102 });
            }

            #[test]
            fn handles_empty_source() {
                let source = Source::new("testfile1", "");
                assert_eq!(source.map_offset(115), LineCol { line: 1, col: 116 });
            }
        }
    }

    mod source_store {
        use super::*;

        mod map_offset {
            use super::*;
            #[test]
            fn calculates_line_and_col_correctly() {
                let mut sources = SourceStore::new();
                let source_id = sources.add_source("testfile", "\n\n\n3456789A\n\nDEFGHIJKL\n\n\n");
                let source_location = sources.map_offset(source_id, 15).unwrap();
                assert_eq!(source_location.line, 6);
                assert_eq!(source_location.col, 3);
                assert_eq!(source_location.source.filename, "testfile");
            }

            #[test]
            fn handles_multiple_files() {
                let mut sources = SourceStore::new();
                let source1_id =
                    sources.add_source("testfile1", "\n\n\n3456789A\n\nDEFGHIJKL\n\n\n");
                let source2_id = sources.add_source("testfile2", "012345\n\n\n");
                let source3_id = sources.add_source("testfile3", "0\n2\n456789ABC\n\n\n");

                let source_location = sources.map_offset(source2_id, 3).unwrap();
                assert_eq!(source_location.line, 1);
                assert_eq!(source_location.col, 4);
                assert_eq!(source_location.source.filename, "testfile2");

                let source_location = sources.map_offset(source3_id, 3).unwrap();
                assert_eq!(source_location.line, 2);
                assert_eq!(source_location.col, 2);
                assert_eq!(source_location.source.filename, "testfile3");

                let source_location = sources.map_offset(source1_id, 3).unwrap();
                assert_eq!(source_location.line, 4);
                assert_eq!(source_location.col, 1);
                assert_eq!(source_location.source.filename, "testfile1");
            }

            #[test]
            fn handles_no_files() {
                let sources = SourceStore::new();
                assert_eq!(sources.map_offset(0, 3), None);
            }
        }

        mod map_span {
            use super::*;

            #[test]
            fn maps_line_and_col_correctly() {
                let mut sources = SourceStore::new();
                let source_id = sources.add_source("testfile", "0\n2\n456789ABC\n\n\n");
                let span = SourceIDSpan::new(source_id, Range { start: 2, end: 6 });

                let SourceSpan {
                    source,
                    start_line,
                    start_col,
                    end_line,
                    end_col,
                } = sources.map_span(&span);

                assert_eq!(source.filename, "testfile");
                assert_eq!(start_line, 2);
                assert_eq!(start_col, 1);
                assert_eq!(end_line, 3);
                assert_eq!(end_col, 3);
            }
        }
    }

    mod source_location {
        use super::*;

        #[test]
        fn first_char_formatted_correctly() {
            let mut sources = SourceStore::new();
            let source_id = sources.add_source("testfile1", "01234567\n\nthing(){}\n\n\n");
            let location = sources.map_offset(source_id, 0).unwrap();
            assert_eq!(location.to_string(), "testfile1: 1:1")
        }

        #[test]
        fn end_of_line_formatted_correctly() {
            let mut sources = SourceStore::new();
            let source_id = sources.add_source("testfile1", "01234567\n\nthing(){}\n\n\n");
            let location = sources.map_offset(source_id, 8).unwrap();
            assert_eq!(location.to_string(), "testfile1: 1:9")
        }

        #[test]
        fn formatted_correctly() {
            let mut sources = SourceStore::new();
            let source_id = sources.add_source("testfile1", "01234567\n\nABCDEFGHI\n\n\n");
            let location = sources.map_offset(source_id, 11).unwrap();
            assert_eq!(location.to_string(), "testfile1: 3:2")
        }
    }

    mod source_span {
        use super::*;

        #[test]
        fn first_char_formatted_correctly() {
            let mut sources = SourceStore::new();
            let source_id = sources.add_source("testfile1", "01234567\n\nABCDEFGHI\n\n\n");
            let span = SourceIDSpan::new(source_id, Range { start: 2, end: 15 });

            let source_span = sources.map_span(&span);
            assert_eq!(source_span.to_string(), "testfile1: 1:3-3:6")
        }
    }
}
