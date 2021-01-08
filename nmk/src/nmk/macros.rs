#[rustfmt::skip]
#[macro_export]
macro_rules! one_hot {
    ($e: expr) => {if $e {"1"} else {"0"}};
}

#[rustfmt::skip]
#[macro_export]
macro_rules! on_off {
    ($e: expr) => {if $e {"on"} else {"off"}};
}

#[cfg(test)]
mod tests {
    #[test]
    fn one_hot() {
        assert_eq!(one_hot!(true), "1");
        assert_eq!(one_hot!(false), "0");
    }

    #[test]
    fn on_off() {
        assert_eq!(on_off!(true), "on");
        assert_eq!(on_off!(false), "off");
    }
}

/// Implement From<Error> to convert any error to Error with caller info
macro_rules! impl_from_error_with_caller {
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
