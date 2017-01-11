use file;
use std::net;

pub fn get_conn(mode: file::Mode) -> Result<Vec<file::Entry>, file::ParseError> {
    file::parse_proc_file(&file::get_path_from_mode(&mode), mode)
}

pub fn get_tcp6() -> Result<Vec<file::Entry>, file::ParseError> {
    get_conn(file::Mode::Tcp6)
}

pub fn get_tcp() -> Result<Vec<file::Entry>, file::ParseError> {
    get_conn(file::Mode::Tcp)
}

pub fn get_udp() -> Result<Vec<file::Entry>, file::ParseError> {
    get_conn(file::Mode::Udp)
}

pub fn get_udp6() -> Result<Vec<file::Entry>, file::ParseError> {
    get_conn(file::Mode::Udp6)
}

pub fn filter_by(entries: &Vec<file::Entry>,
                 mode: &file::Mode,
                 local_address: Option<net::IpAddr>,
                 remote_address: Option<net::IpAddr>,
                 local_port: Option<u32>,
                 remote_port: Option<u32>) -> Vec<file::Entry> {
    entries.iter()
        .cloned()
        .filter(|e| {
            if let Some(l) = local_address {
                l == e.local_address
            }
            else {
                true
            }})
        .filter(|e| {
            if let Some(l) = remote_address {
                l == e.remote_address
            }
            else {
                true
            }})
        .filter(|e| {
            e.mode == *mode
        })
        .filter(|e| {
            if let Some(p) = local_port {
                e.local_port == p
            }
            else {
                true
            }})
        .filter(|e| {
            if let Some(p) = remote_port {
                e.remote_port == p
            }
            else {
                true
            }
        })
        .collect::<Vec<file::Entry>>()
}

#[cfg(test)]
mod tests {

    use std::net;
    use file;

    #[test]
    fn filter_by_test() {
        let entries = vec![
            file::Entry {
                local_address: net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)),
                remote_address: net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)),
                local_port: 80,
                mode: file::Mode::Tcp,
                remote_port: 90,
                connection_state: 0xA,
                uid: 1000},
            file::Entry {
                local_address: net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 2)),
                remote_address: net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 3)),
                local_port: 81,
                mode: file::Mode::Tcp,
                remote_port: 90,
                connection_state: 0xA,
                uid: 1000},
            file::Entry {
                local_address: net::IpAddr::V6(net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
                remote_address: net::IpAddr::V6(net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
                local_port: 80,
                mode: file::Mode::Tcp6,
                remote_port: 90,
                connection_state: 0xA,
                uid: 1000}];
        let result = super::filter_by(&entries, &file::Mode::Tcp, None, None, None, None);
        assert_eq!(result.len(), 2);

        let result = super::filter_by(&entries, &file::Mode::Tcp, Some(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1))), None, None, None);
        assert_eq!(result.len(), 1);
    }

}
