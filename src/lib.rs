mod matrix;
mod vector;

pub use matrix::{multiply, Matrix};
pub use vector::{dot_product, Vector};

#[cfg(test)]
mod tests {
    #[test]
    fn test_fn() {}
}
