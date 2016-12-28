
type Ip6 = [u32; 4];
type Ip4 = u32;

#[derive(Clone, Debug)]
pub enum Ip {
    V4(Ip4),
    V6(Ip6)
}

/// Format an ipv4 address
///
/// # Example
///
/// let result = shift_ipv4(Ox010000FF); => OxFF000001
///
pub fn shift_ipv4(ip: Ip4) -> Ip4 {
    let fmt_ui = (ip << 24 & 0xFF000000) + (ip << 8 & 0x00FF0000) + (ip >> 8 & 0x0000FF00) + (ip >> 24 & 0x000000FF);
    fmt_ui
}


/// Format an ipv6 address
///
/// # Example
///
/// let result = shift_ipv6([0, 0, 0, 0x01000000]); => [0, 0, 0, 1]
///
pub fn shift_ipv6(ip: Ip6) -> Ip6 {
    let mut result: Ip6 = [0, 0, 0, 0];
    let mut count = 0;
    for octets in &ip {
        result[count] = shift_ipv4(*octets);
        count = count + 1;
    }
    result
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

    #[test]
    fn shift_ipv6_test() {
        let array: super::Ip6 = [0, 0, 0, 0x01000000];
        assert_eq!([0, 0, 0, 1], super::shift_ipv6(array));
        let array: super::Ip6 = [0x98765432, 0, 0x12345678, 0x01000000];
        assert_eq!([0x32547698, 0, 0x78563412, 1], super::shift_ipv6(array));
    }
}

