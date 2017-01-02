use std::fs;
use std::io::prelude::*;
use std::io;
use std::num;
use ip;
use util;

#[derive(Clone, Debug)]
pub enum Mode {
    Tcp,
    Udp,
    Tcp6,
    Udp6
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub local_address: ip::Ip,
    pub local_port: u32,
    pub remote_address: ip::Ip,
    pub remote_port: u32,
    pub connection_state: i32,
    pub uid: i32,
    pub mode: Mode,
}

#[derive(Debug)]
pub enum ParseError {
    ParseInt(num::ParseIntError),
    Io(io::Error)
}

impl From<num::ParseIntError> for ParseError {
    fn from(err: num::ParseIntError) -> ParseError {
        ParseError::ParseInt(err)
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::Io(err)
    }
}


pub fn get_path_from_mode(mode: &Mode) -> String {
    let path = match *mode {
        Mode::Tcp => "/proc/net/tcp",
        Mode::Tcp6 => "/proc/net/tcp6",
        Mode::Udp => "/proc/net/udp",
        Mode::Udp6 => "/proc/net/udp6",
    };
    path.to_owned()
}


/// Parse the /proc/net/tcp or /proc/net/udp files (or ipv6 equivalents). Returns a Vec of Entry
///
/// # Example
///
/// let result = parse_proc_file("/proc/net/tcp");
///
pub fn parse_proc_file(path: &str, mode: Mode) -> Result<Vec<Entry>, ParseError> {
    let file = try!(fs::File::open(path));
    let reader = io::BufReader::new(file);
    let mut result: Vec<Entry> = Vec::new();
    let mut lines = reader.lines();
    lines.next();
    for l in lines {
        let line = try!(l);
        let line_vec: Vec<&str> = util::split_and_remove_empty(&line);
        let local: Vec<&str> = line_vec[1].split(':').collect();
        let remote: Vec<&str> = line_vec[2].split(':').collect();
        let (local_addr, remote_addr) = match mode {
            Mode::Tcp|Mode::Udp => {
                (ip::Ip::V4(ip::shift_ipv4(u32::from_str_radix(local[0], 16)?)),
                 ip::Ip::V4(ip::shift_ipv4(u32::from_str_radix(remote[0], 16)?)))
            },
            Mode::Tcp6|Mode::Udp6 => {
                let ip6_local_arr: ip::Ip6 =
                    [u32::from_str_radix(&local[0][0..8], 16)?,
                     u32::from_str_radix(&local[0][8..16], 16)?,
                     u32::from_str_radix(&local[0][16..24], 16)?,
                     u32::from_str_radix(&local[0][24..32], 16)?];
                let ip6_remote_arr: ip::Ip6 =
                    [u32::from_str_radix(&remote[0][0..8], 16)?,
                     u32::from_str_radix(&remote[0][8..16], 16)?,
                     u32::from_str_radix(&remote[0][16..24], 16)?,
                     u32::from_str_radix(&remote[0][24..32], 16)?];
                (ip::Ip::V6(ip::shift_ipv6(ip6_local_arr)),
                 ip::Ip::V6(ip::shift_ipv6(ip6_remote_arr)))
            }
        };
        let local_port = u32::from_str_radix(local[1], 16)?;
        let remote_port = u32::from_str_radix(remote[1], 16)?;
        let uid = line_vec[7].parse::<i32>()?;
        let conn_state = i32::from_str_radix(line_vec[3], 16)?;
        result.push(Entry {
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
    use ip;

    #[test]
    fn parse_ipv4_file_test() {
        let mut path = env::current_dir().unwrap().to_str().unwrap_or("").to_string();
        path.push_str("/test/static/linux_tcp_4");
        let result = super::parse_proc_file(&path, super::Mode::Tcp).unwrap();
        assert_eq!(result.len(), 3);
        let e0 = &result[0];
        assert_eq!(if let ip::Ip::V4(val) = e0.local_address { val } else { panic!() }, 0x7F000001);
        assert_eq!(e0.local_port, 0x19);
        assert_eq!(if let ip::Ip::V4(val) = e0.remote_address { val } else { panic!() }, 0);
        assert_eq!(e0.remote_port, 0);
        assert_eq!(e0.uid, 0);
        assert_eq!(e0.connection_state, 0xA);

        let e1 = &result[1];
        assert_eq!(if let ip::Ip::V4(val) = e1.local_address { val } else { panic!() }, 0x7F000001);
        assert_eq!(e1.local_port, 0x8AE);
        assert_eq!(if let ip::Ip::V4(val) = e1.remote_address { val } else { panic!() }, 0);
        assert_eq!(e1.remote_port, 0);
        assert_eq!(e1.uid, 1000);
        assert_eq!(e1.connection_state, 0xA);

        let e2 = &result[2];
        assert_eq!(if let ip::Ip::V4(val) = e2.local_address { val } else { panic!() }, 0);
        assert_eq!(e2.local_port, 0x006F);
        assert_eq!(if let ip::Ip::V4(val) = e2.remote_address { val } else { panic!() }, 0x7F000203);
        assert_eq!(e2.remote_port, 0);
        assert_eq!(e2.uid, 0);
        assert_eq!(e2.connection_state, 0xA);
    }

    #[test]
    fn parse_ipv6_file_test() {
        let mut path = env::current_dir().unwrap().to_str().unwrap_or("").to_string();
        path.push_str("/test/static/linux_tcp_6");
        let result = super::parse_proc_file(&path, super::Mode::Tcp6).unwrap();
        assert_eq!(result.len(), 7);

        let e0 = &result[0];
        assert_eq!(if let ip::Ip::V6(val) = e0.local_address { val } else { panic!() },[0, 0, 0, 0]);
        assert_eq!(e0.local_port, 0x22B8);
        assert_eq!(if let ip::Ip::V6(val) = e0.remote_address { val } else { panic!() }, [0, 0, 0, 0]);
        assert_eq!(e0.remote_port, 0);
        assert_eq!(e0.uid, 999);
        assert_eq!(e0.connection_state, 0xA);

        let e6 = &result[6];
        assert_eq!(if let ip::Ip::V6(val) = e6.local_address { val } else { panic!() },[0x2a01cb15, 0x80543e00, 0x5ee0c5ff, 0xfe50c693]);
        assert_eq!(e6.local_port, 0xAB3E);
        assert_eq!(if let ip::Ip::V6(val) = e6.remote_address { val } else { panic!() }, [0x2a001450, 0x400C0C01, 0, 0x5E]);
        assert_eq!(e6.remote_port, 0x01BB);
        assert_eq!(e6.uid, 1000);
        assert_eq!(e6.connection_state, 1);

    }


    #[test]
    fn parse_udp_4_file_test() {
        let mut path = env::current_dir().unwrap().to_str().unwrap_or("").to_string();
        path.push_str("/test/static/linux_udp_4");
        let result = super::parse_proc_file(&path, super::Mode::Tcp).unwrap();
        assert_eq!(result.len(), 3);
        let e0 = &result[0];
        assert_eq!(if let ip::Ip::V4(val) = e0.local_address { val } else { panic!() }, 0);
        assert_eq!(e0.local_port, 0x9B25);
        assert_eq!(if let ip::Ip::V4(val) = e0.remote_address { val } else { panic!() }, 0);
        assert_eq!(e0.remote_port, 0);
        assert_eq!(e0.uid, 0);
        assert_eq!(e0.connection_state, 7);

        let e1 = &result[1];
        assert_eq!(if let ip::Ip::V4(val) = e1.local_address { val } else { panic!() }, 0);
        assert_eq!(e1.local_port, 0x006F);
        assert_eq!(if let ip::Ip::V4(val) = e1.remote_address { val } else { panic!() }, 0);
        assert_eq!(e1.remote_port, 0);
        assert_eq!(e1.uid, 0);
        assert_eq!(e1.connection_state, 7);

        let e2 = &result[2];
        assert_eq!(if let ip::Ip::V4(val) = e2.local_address { val } else { panic!() }, 0x7F000001);
        assert_eq!(e2.local_port, 0x00A1);
        assert_eq!(if let ip::Ip::V4(val) = e2.remote_address { val } else { panic!() }, 0);
        assert_eq!(e2.remote_port, 0);
        assert_eq!(e2.uid, 0);
        assert_eq!(e2.connection_state, 7);
    }

}
