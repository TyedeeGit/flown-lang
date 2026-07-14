#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    pub line: usize,
    pub column: usize,
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