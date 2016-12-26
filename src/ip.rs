
/// Format an ipv4 address from /proc/net/*
///
/// # Example
///
/// let result = shift_ipv4(Ox010000FF); => OxFF000001
///
pub fn shift_ipv4(ip: u64) -> u64 {
    let fmt_ui = (ip << 24 & 0xFF000000) + (ip << 8 & 0x00FF0000) + (ip >> 8 & 0x0000FF00) + (ip >> 24 & 0x000000FF);
    fmt_ui
}


#[cfg(test)]
mod tests {

    #[test]
    fn shift_ipv4_test() {
        assert_eq!(0xFF000001, super::shift_ipv4(0x010000FF));
        assert_eq!(0x12345678, super::shift_ipv4(0x78563412));
        assert_eq!(0xFA000000, super::shift_ipv4(0x000000FA));
        assert_eq!(0x000000FA, super::shift_ipv4(0xFA000000));
    }
}

