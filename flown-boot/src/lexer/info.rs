#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span<'a> {
    pub start: Location,
    pub end: Location,
    pub sl: &'a str
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Spanning<'a, Inner> {
    pub span: Span<'a>,
    pub inner: Inner,
}