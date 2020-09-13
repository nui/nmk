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
