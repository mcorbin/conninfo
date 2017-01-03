use file;

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

pub fn filter_by(entries: &Vec<file::Entry>, mode: &file::Mode, local_address: Option<String>, remote_address: Option<String>, local_port: u32, remote_port: u32) {

}
