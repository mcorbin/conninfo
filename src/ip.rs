use std::num;
use std::net;

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

/// Format an ipv4 address string from /proc/net and returns a Ipv4Addr
///
/// # Example
///
/// let result = proc_str_to_ip4("010000FF"); => IpAddr for OxFF000001
///
pub fn proc_str_to_ip4(ip_string: &str) -> Result<net::IpAddr, IpError> {
    let ip = u32::from_str_radix(ip_string, 16)?;
    let p1 = (ip & 0x000000FF) as u8;
    let p2 = (ip >> 8 & 0x000000FF) as u8;
    let p3 = (ip >> 16 & 0x000000FF) as u8;
    let p4 = (ip >> 24 & 0x000000FF) as u8;
    Ok(net::IpAddr::V4(net::Ipv4Addr::new(p1, p2, p3, p4)))
}

/// Format an ipv6 address string from /proc/net and returns a Ipv6Addr
///
/// # Example
///
/// let result = proc_str_to_ip4("15CB012A003E5480FFC5E05E93C650FE"); => IpAddr for Ox2A001450400C0C010000000000000053
///
pub fn proc_str_to_ip6(ip_string: &str) -> Result<net::IpAddr, IpError> {
    let p1 = u16::from_str_radix(&ip_string[0..2], 16)?;
    let p2 = u16::from_str_radix(&ip_string[2..4], 16)?;
    let p3 = u16::from_str_radix(&ip_string[4..6], 16)?;
    let p4 = u16::from_str_radix(&ip_string[6..8], 16)?;
    let p5 = u16::from_str_radix(&ip_string[8..10], 16)?;
    let p6 = u16::from_str_radix(&ip_string[10..12], 16)?;
    let p7 = u16::from_str_radix(&ip_string[12..14], 16)?;
    let p8 = u16::from_str_radix(&ip_string[14..16], 16)?;
    let p9 = u16::from_str_radix(&ip_string[16..18], 16)?;
    let p10 = u16::from_str_radix(&ip_string[18..20], 16)?;
    let p11 = u16::from_str_radix(&ip_string[20..22], 16)?;
    let p12 = u16::from_str_radix(&ip_string[22..24], 16)?;
    let p13 = u16::from_str_radix(&ip_string[24..26], 16)?;
    let p14 = u16::from_str_radix(&ip_string[26..28], 16)?;
    let p15 = u16::from_str_radix(&ip_string[28..30], 16)?;
    let p16 = u16::from_str_radix(&ip_string[30..32], 16)?;
    Ok(net::IpAddr::V6(net::Ipv6Addr::new((p4 << 8) + p3, (p2 << 8) + p1, (p8 << 8) + p7, (p6 << 8) + p5, (p12 << 8) + p11, (p10 << 8) + p9, (p16 << 8) + p15, (p14 << 8) + p13)))
}

/// Convert an ipv4 string into an Ipv4Addr
///
/// # Example
///
/// let result = str_to_ipv4("127.0.0.1")? => Ipv4Addr 0x7F000001
///
pub fn str_to_ip4(ip: &str) -> Result<net::Ipv4Addr, IpError> {
    let result = match ip {
        "localhost" => Ok(net::Ipv4Addr::new(127, 0, 0, 1)),
        _ => {
            let ip_vec: Vec<&str> = ip.split('.').collect();
            if ip_vec.len() == 4 {
                Ok(net::Ipv4Addr::new(u8::from_str_radix(ip_vec[0], 10)?,
                                   u8::from_str_radix(ip_vec[1], 10)?,
                                   u8::from_str_radix(ip_vec[2], 10)?,
                                   u8::from_str_radix(ip_vec[3], 10)?))
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

    use std::net;

    #[test]
    fn proc_str_to_ip4_test() {
        assert_eq!(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)) , super::proc_str_to_ip4("0100007F").unwrap());
        assert_eq!(net::IpAddr::V4(net::Ipv4Addr::new(0x12, 0x34, 0x56, 0x78)) , super::proc_str_to_ip4("78563412").unwrap());
        assert_eq!(net::IpAddr::V4(net::Ipv4Addr::new(0xFA, 0, 0, 0)) , super::proc_str_to_ip4("000000FA").unwrap());
        assert_eq!(net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0xFA)) , super::proc_str_to_ip4("FA000000").unwrap());
    }

    #[test]
    fn proc_str_to_ip6_test() {
        assert_eq!(net::IpAddr::V6(net::Ipv6Addr::new(0x3254, 0x7698, 0, 0, 0x7856, 0x3412, 0, 1)) , super::proc_str_to_ip6("98765432000000001234567801000000").unwrap());
    }

    #[test]
    fn str_to_ip4_test() {
        assert_eq!(net::Ipv4Addr::new(127, 0, 0, 1), super::str_to_ip4("localhost").unwrap());
        assert_eq!(net::Ipv4Addr::new(127, 0, 0, 1), super::str_to_ip4("127.0.0.1").unwrap());
        assert_eq!(net::Ipv4Addr::new(0, 0, 0, 0), super::str_to_ip4("0.0.0.0").unwrap());
        assert_eq!(net::Ipv4Addr::new(255, 255, 255, 255), super::str_to_ip4("255.255.255.255").unwrap());
    }
}

