use std::fs;
use std::io::prelude::*;
use std::io;
use std::num;
use ip;
use util;
use info;

#[derive(Debug)]
pub enum ParseError {
    ParseInt(num::ParseIntError),
    Io(io::Error),
    Ip(ip::IpError)
}

impl From<num::ParseIntError> for ParseError {
    fn from(err: num::ParseIntError) -> ParseError {
        ParseError::ParseInt(err)
    }
}

impl From<ip::IpError> for ParseError {
    fn from(err: ip::IpError) -> ParseError {
        ParseError::Ip(err)
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::Io(err)
    }
}

pub fn get_path_from_mode(mode: &info::Mode) -> String {
    let path = match *mode {
        info::Mode::Tcp => "/proc/net/tcp",
        info::Mode::Tcp6 => "/proc/net/tcp6",
        info::Mode::Udp => "/proc/net/udp",
        info::Mode::Udp6 => "/proc/net/udp6",
    };
    path.to_owned()
}

/// Parse the /proc/net/tcp or /proc/net/udp files (or ipv6 equivalents). Returns a Vec of Entry
///
/// # Example
///
/// let result = parse_proc_file("/proc/net/tcp", Mode::Tcp);
///
pub fn parse_proc_file(path: &str, mode: info::Mode) -> Result<Vec<info::Entry>, ParseError> {
    let file = try!(fs::File::open(path));
    let reader = io::BufReader::new(file);
    let mut result: Vec<info::Entry> = Vec::new();
    let mut lines = reader.lines();
    lines.next();
    for l in lines {
        let line = try!(l);
        let line_vec: Vec<&str> = util::split_and_remove_empty(&line);
        let local: Vec<&str> = line_vec[1].split(':').collect();
        let remote: Vec<&str> = line_vec[2].split(':').collect();
        let (local_addr, remote_addr) = match mode {
            info::Mode::Tcp|info::Mode::Udp => {
                (ip::proc_str_to_ip4(local[0])?,
                 ip::proc_str_to_ip4(remote[0])?)
            },
            info::Mode::Tcp6|info::Mode::Udp6 => {
                (ip::proc_str_to_ip6(local[0])?,
                 ip::proc_str_to_ip6(remote[0])?)

            }
        };
        let local_port = u32::from_str_radix(local[1], 16)?;
        let remote_port = u32::from_str_radix(remote[1], 16)?;
        let uid = line_vec[7].parse::<i32>()?;
        let conn_state = i32::from_str_radix(line_vec[3], 16)?;
        result.push(info::Entry {
            local_address: local_addr,
            local_port: local_port,
            remote_address: remote_addr,
            remote_port: remote_port,
            uid: uid,
            connection_state: conn_state,
            mode: mode.clone()
        });
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::net;
    use info;

    #[test]
    fn parse_tcp_4_file_test() {
        let mut path = env::current_dir().unwrap().to_str().unwrap_or("").to_string();
        path.push_str("/test/static/linux_tcp_4");
        let result = super::parse_proc_file(&path, info::Mode::Tcp).unwrap();
        assert_eq!(result.len(), 3);
        let e0 = &result[0];
        assert_eq!(e0.local_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(e0.local_port, 0x19);
        assert_eq!(e0.remote_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(e0.remote_port, 0);
        assert_eq!(e0.uid, 0);
        assert_eq!(e0.connection_state, 0xA);

        let e1 = &result[1];
        assert_eq!(e1.local_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(e1.local_port, 0x8AE);
        assert_eq!(e0.remote_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(e1.remote_port, 0);
        assert_eq!(e1.uid, 1000);
        assert_eq!(e1.connection_state, 0xA);

        let e2 = &result[2];
        assert_eq!(e2.local_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(e2.local_port, 0x006F);
        assert_eq!(e2.remote_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(0x7F, 0, 2, 3)));
        assert_eq!(e2.remote_port, 0);
        assert_eq!(e2.uid, 0);
        assert_eq!(e2.connection_state, 0xA);
    }

    #[test]
    fn parse_tcp_6_file_test() {
        let mut path = env::current_dir().unwrap().to_str().unwrap_or("").to_string();
        path.push_str("/test/static/linux_tcp_6");
        let result = super::parse_proc_file(&path, info::Mode::Tcp6).unwrap();
        assert_eq!(result.len(), 7);

        let e0 = &result[0];
        assert_eq!(e0.local_address,
                   net::IpAddr::V6(net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)));
        assert_eq!(e0.local_port, 0x22B8);
        assert_eq!(e0.remote_address,
                   net::IpAddr::V6(net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)));
        assert_eq!(e0.remote_port, 0);
        assert_eq!(e0.uid, 999);
        assert_eq!(e0.connection_state, 0xA);

        let e6 = &result[6];
        assert_eq!(e6.local_address,
                   net::IpAddr::V6(net::Ipv6Addr::new(0x2a01, 0xcb15, 0x8054, 0x3e00, 0x5ee0, 0xc5ff, 0xfe50, 0xc693)));
        assert_eq!(e6.local_port, 0xAB3E);
        assert_eq!(e6.remote_address,
                   net::IpAddr::V6(net::Ipv6Addr::new(0x2a00, 0x1450, 0x400C, 0x0C01, 0, 0, 0, 0x5E)));
        assert_eq!(e6.remote_port, 0x01BB);
        assert_eq!(e6.uid, 1000);
        assert_eq!(e6.connection_state, 1);
    }


    #[test]
    fn parse_udp_4_file_test() {
        let mut path = env::current_dir().unwrap().to_str().unwrap_or("").to_string();
        path.push_str("/test/static/linux_udp_4");
        let result = super::parse_proc_file(&path, info::Mode::Tcp).unwrap();
        assert_eq!(result.len(), 3);
        let e0 = &result[0];
        assert_eq!(e0.local_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(e0.local_port, 0x9B25);
        assert_eq!(e0.remote_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(e0.remote_port, 0);
        assert_eq!(e0.uid, 0);
        assert_eq!(e0.connection_state, 7);

        let e1 = &result[1];
        assert_eq!(e1.local_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(e1.local_port, 0x006F);
        assert_eq!(e1.remote_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(e1.remote_port, 0);
        assert_eq!(e1.uid, 0);
        assert_eq!(e1.connection_state, 7);

        let e2 = &result[2];
        assert_eq!(e2.local_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(e2.local_port, 0x00A1);
        assert_eq!(e2.remote_address,
                   net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(e2.remote_port, 0);
        assert_eq!(e2.uid, 0);
        assert_eq!(e2.connection_state, 7);
    }


    #[test]
    fn parse_udp6_file_test() {
        let mut path = env::current_dir().unwrap().to_str().unwrap_or("").to_string();
        path.push_str("/test/static/linux_udp_6");

        let result = super::parse_proc_file(&path, info::Mode::Tcp6).unwrap();
        assert_eq!(result.len(), 3);
        let e0 = &result[0];
        assert_eq!(e0.local_address,
                   net::IpAddr::V6(net::Ipv6Addr::new(0xfe80, 0, 0, 0, 0x5ee0, 0xc5ff, 0xfe50, 0xc693)));
        assert_eq!(e0.local_port, 0x1A73);
        assert_eq!(e0.remote_address,
                   net::IpAddr::V6(net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)));
        assert_eq!(e0.remote_port, 0);
        assert_eq!(e0.uid, 1000);
        assert_eq!(e0.connection_state, 0x7);

        let e2 = &result[2];
        assert_eq!(e2.local_address,
                   net::IpAddr::V6(net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
        assert_eq!(e2.local_port, 0x1A73);
        assert_eq!(e2.remote_address,
                   net::IpAddr::V6(net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)));
        assert_eq!(e2.remote_port, 0);
        assert_eq!(e2.uid, 1000);
        assert_eq!(e2.connection_state, 0x7);

    }

}
