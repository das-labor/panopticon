use std::ops::Range;

use Result;
use il::rreil::Statement;

/// A sequence of statements in an Intermediate Language. Implement this for your new IL
pub trait Language {
    /// A statement of this intermediate language; it currently must be able to convert itself from
    /// standard panopticon RREIL
    type Statement: From<Statement>;
    /// Add a statement
    fn push(&mut self, statement: Self::Statement) -> Result<usize>;
    /// Add several statements
    fn append(&mut self, statements: Vec<Self::Statement>) -> Result<Range<usize>>;
    /// How many statements
    fn len(&self) -> usize;
    /// Hint for number of unique strings or variables in this sequence
    fn number_of_strings(&self) -> Option<usize> {
        None
    }
}

/// Language is generically implemented for any Vector of statements, when those statements
/// can be converted from a RREIL statement, and is also cloneable.
///
/// Therefore, if you have some `Statement` type and you impl `From<core::Statement> for Statement,
/// then you need only write `Function::<Vec<Statement>>::new::<Arch>`
impl<S: Clone + From<Statement>> Language for Vec<S> {
    type Statement = S;
    fn push(&mut self, statement: Self::Statement) -> Result<usize> {
        self.push(statement);
        Ok(1)
    }
    fn append(&mut self, statements: Vec<Self::Statement>) -> Result<Range<usize>> {
        let start = self.len();
        let len = statements.len();
        self.extend(statements);
        Ok(start..start+len)
    }
    fn len(&self) -> usize {
        self.len()
    }
}

/// A trait for how to iterate over statements of the IL for a given `Language`
pub trait StatementIterator<Item> {
    type IntoIter: Iterator<Item=Item>;
    /// Iterate over a `range` of statements
    fn iter_statements(self, range: Range<usize>) -> Self::IntoIter;
}

impl<'a, S: Clone + From<Statement>> StatementIterator<S> for &'a Vec<S> {
    type IntoIter = ::std::iter::Cloned<::std::slice::Iter<'a, S>>;

    fn iter_statements(self, range: Range<usize>) -> Self::IntoIter {
        let i = self[range].iter().cloned();
        i
    }
}
