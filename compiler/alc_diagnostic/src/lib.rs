pub use codespan::{FileId, Files};
pub use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::term;
use log::debug;
use std::ops::Deref;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Span(codespan::Span);

#[derive(Debug)]
pub struct Spanned<T>(Span, T);

impl Into<codespan::Span> for Span {
    #[inline]
    fn into(self) -> codespan::Span {
        self.0
    }
}

impl From<codespan::Span> for Span {
    #[inline]
    fn from(span: codespan::Span) -> Span {
        Span(span)
    }
}

impl Deref for Span {
    type Target = codespan::Span;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Span {
    #[inline]
    pub fn new(start: impl Into<codespan::ByteIndex>, end: impl Into<codespan::ByteIndex>) -> Span {
        codespan::Span::new(start, end).into()
    }

    #[inline]
    pub fn dummy() -> Span {
        codespan::Span::initial().into()
    }

    #[inline]
    pub fn span<T>(self, t: T) -> Spanned<T> {
        Spanned(self, t)
    }

    #[inline]
    pub fn clip(self) -> Span {
        codespan::Span::new(self.0.end(), self.0.end()).into()
    }

    #[inline]
    pub fn merge(self, other: Span) -> Span {
        self.0.merge(other.0).into()
    }
}

impl<T> Spanned<T> {
    #[inline]
    pub fn into_raw(self) -> T {
        self.1
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.0
    }

    #[inline]
    pub fn respan(self, span: Span) -> Spanned<T> {
        span.span(self.1)
    }

    #[inline]
    pub fn boxed(self) -> Spanned<Box<T>> {
        self.0.span(Box::new(self.1))
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

pub type Result<T> = std::result::Result<T, Diagnostic>;

pub fn emit(files: &Files, diagnostic: &Diagnostic) {
    let writer = term::termcolor::StandardStream::stderr(term::termcolor::ColorChoice::Always);
    let config = term::Config::default();
    debug!("trying to emit diagnostic {:?}", diagnostic);
    term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
}
