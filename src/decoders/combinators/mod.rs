mod then;
mod then_try;
mod chain;

pub use then::{Then, ThenFnPtr};
pub use then_try::{ThenTry, ThenTryFnPtr};
pub use chain::Chain;

/// A sum type representing a value that can be one of two types.
///
/// This is used for error handling in combinators where an error
/// can come from either the first operation (Left) or the second operation (Right).
/// Allows for automatic error type creation by combining the sub errors into one,
/// a generic error combiner. But this places a burden on the caller to then match
/// against the two (and possibly nested) possibilities.
///
/// ## What is a Sum Type?
///
/// A "sum type" (also called a "tagged union" or "variant type") is a data type
/// that can hold a value of one of several different types. Unlike a "product type"
/// (like a struct) that holds values of ALL its component types simultaneously,
/// a sum type holds a value of exactly ONE of its component types at any given time.
///
/// Examples:
/// - `Option<T>` is a sum type: either `Some(T)` OR `None`
/// - `Result<T, E>` is a sum type: either `Ok(T)` OR `Err(E)`
/// - `Either<L, R>` is a sum type: either `Left(L)` OR `Right(R)`
///
/// Sum types are called "sum" because the number of possible values is the
/// sum of the possible values of each variant, not the product.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Either<L, R> {
    /// The left variant
    Left(L),
    /// The right variant
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Create a Left variant
    pub const fn left(value: L) -> Self {
        Either::Left(value)
    }
    
    /// Create a Right variant  
    pub const fn right(value: R) -> Self {
        Either::Right(value)
    }
}
