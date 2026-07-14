#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Spanning<Inner> {
    pub span: Span,
    pub inner: Inner,
}