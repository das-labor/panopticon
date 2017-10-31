use std::ops::Range;

use Result;
use il::{Language, CallIterator, Statement, StatementIterator, LoadStatement};

#[derive(Clone)]
pub struct NoopStatement(());

#[derive(Default)]
pub struct Noop {}

impl From<Statement> for NoopStatement {
    fn from(_: Statement) -> Self {
        NoopStatement(())
    }
}

impl Language for Noop {
    type Statement = NoopStatement;

    fn push(&mut self, _statement: Self::Statement) -> Result<usize> {
        Ok(0)
    }

    fn append(&mut self, _statements: Vec<Self::Statement>) -> Result<Range<usize>> {
        Ok(0..0)
    }

    fn len(&self) -> usize {
        0
    }
}

impl<'a> StatementIterator<NoopStatement> for &'a Noop {
    type IntoIter = ::std::iter::Cloned<::std::slice::Iter<'a, NoopStatement>>;
    fn iter_statements(self, _range: Range<usize>) -> Self::IntoIter {
        [].iter().cloned()
    }
}

impl<'a> CallIterator for &'a Noop {
    type Iter = ::std::iter::Cloned<::std::slice::Iter<'a, u64>>;

    fn iter_calls(self) -> Self::Iter {
        [].iter().cloned()
    }
}

impl LoadStatement for NoopStatement {}
