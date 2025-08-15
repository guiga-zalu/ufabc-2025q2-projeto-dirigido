pub trait Comparison<A> {
    fn compare(a: &A, b: &A) -> f64;
}
