#[inline(always)]
pub fn one_hot(val: bool) -> &'static str {
    if val {
        "1"
    } else {
        "0"
    }
}

#[inline(always)]
pub fn on_off(val: bool) -> &'static str {
    if val {
        "on"
    } else {
        "off"
    }
}
