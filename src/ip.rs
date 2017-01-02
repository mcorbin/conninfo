use std::num;

pub type Ip6 = [u32; 4];
pub type Ip4 = u32;

#[derive(Clone, Debug)]
pub enum Ip {
    V4(Ip4),
    V6(Ip6)
}


#[derive(Debug)]
pub enum IpError {
    ParseInt(num::ParseIntError),
    Format
}

impl From<num::ParseIntError> for IpError {
    fn from(err: num::ParseIntError) -> IpError {
        IpError::ParseInt(err)
    }
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


/// Convert an ipv4 string into u32
///
/// # Example
///
/// let result = ipv4_to_u32("127.0.0.1")? => 0x7F000001
///
pub fn ipv4_to_u32(ip: &str) -> Result<u32, IpError> {
    let result = match ip {
        "localhost" => Ok(0x7F000001),
        _ => {
            let ip_vec: Vec<&str> = ip.split('.').collect();
            if ip_vec.len() == 4 {
                println!("{}", u32::from_str_radix(ip_vec[0], 10)? << 24);
                let res = (u32::from_str_radix(ip_vec[0], 10)? << 24) +
                    (u32::from_str_radix(ip_vec[1], 10)? << 16) +
                    (u32::from_str_radix(ip_vec[2], 10)? << 8) +
                    u32::from_str_radix(ip_vec[3], 10)?;
                Ok(res)
            }
            else {
                Err(IpError::Format)
            }
        }
    };
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

    #[test]
    fn ipv4_to_u32_test() {
        assert_eq!(0x7F000001, super::ipv4_to_u32("localhost").unwrap());
        assert_eq!(0x7F000001, super::ipv4_to_u32("127.0.0.1").unwrap());
        assert_eq!(0, super::ipv4_to_u32("0.0.0.0").unwrap());
        assert_eq!(0xFFFFFFFF, super::ipv4_to_u32("255.255.255.255").unwrap());
    }
}

