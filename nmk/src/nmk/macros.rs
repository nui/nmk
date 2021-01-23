/// Implement From<Error> to convert any error to Error with caller info
macro_rules! impl_from_error {
    ($ty:ty) => {
        impl From<$ty> for crate::error::Error {
            #[track_caller]
            fn from(err: $ty) -> Self {
                Self::new(
                    Box::new(err),
                    stringify!($ty),
                    *::std::panic::Location::caller(),
                )
            }
        }
    };
}
