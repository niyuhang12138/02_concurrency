mod matrix;
mod metrics;
mod vector;

pub use matrix::{multiply, Matrix};
pub use metrics::{AMapMetrics, CMapMetrics};
pub use vector::{dot_product, Vector};

#[cfg(test)]
mod tests {
    #[test]
    fn test_fn() {}
}
