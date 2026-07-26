use std::{fs, io};

#[derive(Debug, Default)]
pub struct OsInfo {
    name: String,
    pretty_name: String,
    id: String,
    build_id: String,
}

pub fn parse_os_release(content: &str) -> io::Result<OsInfo> {
    let mut os_info = OsInfo::default();

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        let value = value.trim().trim_matches('"').to_string();

        match key {
            "NAME" => os_info.name = value,
            "PRETTY_NAME" => os_info.pretty_name = value,
            "ID" => os_info.id = value,
            "BUILD_ID" => os_info.build_id = value,
            _ => {}
        }
    }

    Ok(os_info)
}
