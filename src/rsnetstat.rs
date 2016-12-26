use std::fs;
use std::io::prelude::*;
use std::io;
use std::num;
use ip;

#[derive(Clone, Debug)]
pub struct Entry {
    pub local_address: u64,
    pub local_port: u64,
    pub remote_address: u64,
    pub remote_port: u64,
    pub connection_state: i32,
    pub uid: i32,
}

#[derive(Debug)]
pub enum NetstatError {
    ParseInt(num::ParseIntError),
    Io(io::Error)
}

impl From<num::ParseIntError> for NetstatError {
    fn from(err: num::ParseIntError) -> NetstatError {
        NetstatError::ParseInt(err)
    }
}
impl From<io::Error> for NetstatError {
    fn from(err: io::Error) -> NetstatError {
        NetstatError::Io(err)
    }
}

/// Parse the /proc/net/tcp file. Returns a Vec of Entry
///
/// # Example
///
/// let result = parse_linux_file("/proc/net/tcp");
///
fn parse_linux_file(path: &str) -> Result<Vec<Entry>, NetstatError> {
    let tcp_file = try!(fs::File::open(path));
    let reader = io::BufReader::new(tcp_file);
    let mut result: Vec<Entry> = Vec::new();
    let mut lines = reader.lines();
    lines.next();
    for l in lines {
        let line = try!(l);
        let line_vec: Vec<&str> = line
            .trim().
            split(' ').
            filter(|&v| v != "").
            collect();
        let local: Vec<&str> = line_vec[1].split(':').collect();
        let local_addr = ip::shift_ipv4(u64::from_str_radix(local[0], 16)?);
        let local_port = u64::from_str_radix(local[1], 16)?;
        let remote: Vec<&str> = line_vec[2].split(':').collect();
        let remote_addr = ip::shift_ipv4(u64::from_str_radix(remote[0], 16)?);
        let remote_port = u64::from_str_radix(remote[1], 16)?;
        let uid = line_vec[7].parse::<i32>()?;
        let conn_state = i32::from_str_radix(line_vec[3], 16)?;
        result.push(Entry {
            local_address: local_addr,
            local_port: local_port,
            remote_address: remote_addr,
            remote_port: remote_port,
            uid: uid,
            connection_state: conn_state
        });
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn parse_linux_file_test() {
        let mut path = env::current_dir().unwrap().to_str().unwrap_or("").to_string();
        path.push_str("/test/static/linux_tcp");
        let result = super::parse_linux_file(&path).unwrap();
        assert_eq!(result.len(), 3);
        let e0 = &result[0];
        assert_eq!(e0.local_address, 0x7F000001);
        assert_eq!(e0.local_port, 0x19);
        assert_eq!(e0.remote_address, 0);
        assert_eq!(e0.remote_port, 0);
        assert_eq!(e0.uid, 0);
        assert_eq!(e0.connection_state, 0xA);

        let e1 = &result[1];
        assert_eq!(e1.local_address, 0x7F000001);
        assert_eq!(e1.local_port, 0x8AE);
        assert_eq!(e1.remote_address, 0);
        assert_eq!(e1.remote_port, 0);
        assert_eq!(e1.uid, 1000);
        assert_eq!(e1.connection_state, 0xA);

        let e2 = &result[2];
        assert_eq!(e2.local_address, 0);
        assert_eq!(e2.local_port, 0x006F);
        assert_eq!(e2.remote_address, 0x7F000203);
        assert_eq!(e2.remote_port, 0);
        assert_eq!(e2.uid, 0);
        assert_eq!(e2.connection_state, 0xA);
    }

}

