#[macro_export]
macro_rules! vec2d {
    ( $($($x:expr),+ $(,)?);* $(;)? ) => (
        vec![$(
            vec![$($x),+],
        )*]
    );
}
