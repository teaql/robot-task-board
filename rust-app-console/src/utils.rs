pub struct SystemInfo {
    pub cpu_model: String,
    pub mem_size: String,
}

pub fn get_system_info() -> SystemInfo {
    let mut cpu_model = "Unknown CPU".to_owned();
    let mut mem_size = "Unknown Mem".to_owned();

    // Parse /proc/cpuinfo
    if let Ok(file) = std::fs::File::open("/proc/cpuinfo") {
        use std::io::BufRead;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            if line.starts_with("model name") {
                if let Some(pos) = line.find(':') {
                    cpu_model = line[pos + 1..].trim().to_owned();
                    break;
                }
            } else if line.starts_with("Processor") || line.starts_with("cpu") {
                if let Some(pos) = line.find(':') {
                    let val = line[pos + 1..].trim().to_owned();
                    if !val.is_empty() {
                        cpu_model = val;
                    }
                }
            }
        }
    }

    // Parse /proc/meminfo
    if let Ok(file) = std::fs::File::open("/proc/meminfo") {
        use std::io::BufRead;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            if line.starts_with("MemTotal") {
                if let Some(pos) = line.find(':') {
                    let parts: Vec<&str> = line[pos + 1..].split_whitespace().collect();
                    if !parts.is_empty() {
                        if let Ok(kb) = parts[0].parse::<u64>() {
                            let gb = kb as f64 / 1024.0 / 1024.0;
                            mem_size = format!("{:.1} GB", gb);
                        } else {
                            mem_size = parts[0].to_owned();
                        }
                    }
                    break;
                }
            }
        }
    }

    SystemInfo { cpu_model, mem_size }
}
