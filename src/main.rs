use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
//use binary_to_ascii::convert;
//use regex::Regex;
use std::fs;
use sysinfo::Disks;
//use std::process::Command;

/*fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}*/
fn round_to_nth_digit(x: f64, digits: usize) -> f64 {
    let factor = 10_f64.powi(digits as i32);
    (x * factor).round() / factor
}

fn get_numbers(string_with_numbers_placeholder: String, mut string_with_numbers: String) -> String {
    let numbers: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    for char in string_with_numbers_placeholder.chars() {
        if numbers.contains(&char) {
            string_with_numbers.push(char);
        }
    }
    string_with_numbers
}

fn find_memory_entry(file: String, entry_name: &String) -> String {
    // find index of entry searched
    let location_of_entry: usize = file.find(entry_name).expect("Err");

    // save content beginning at the entry
    let mut entry = &file[location_of_entry..];
    // save location of next kB
    let entry_end_location: usize = entry.find("kB").expect("Err");
    // save only the entry itself
    entry = &entry[..entry_end_location];

    let placeholder_for_numbers: String = String::from("");
    let entry: String = get_numbers(entry.to_string(), placeholder_for_numbers);
    entry
}

fn main() {
    // Memory
    // Open linux Memory Info file
    let mem_file = File::open("/proc/meminfo");
    // create String to push content to
    let mut memory_contents = String::new();
    // push content to created string
    match mem_file
        .expect("REASON")
        .read_to_string(&mut memory_contents)
    {
        Ok(_) => {}
        Err(_) => {
            println!("Error reading file")
        }
    }

    // print content of file
    //println!("{}", memory_contents);

    // find desired entry in string -> convert output to f64 -> kb to gb -> round to nearest 2
    let mut string = String::from("MemTotal");
    let mem_total: String = find_memory_entry(memory_contents.to_string(), &string);
    let mut mem_total: f64 = mem_total.parse::<f64>().expect("Err");
    mem_total = mem_total / 1024.0 / 1024.0;
    mem_total = round_to_nth_digit(mem_total, 5);

    // same
    /*string.clear();
    string.push_str("MemFree");
    let mem_free: String=find_memory_entry(memory_contents.to_string(),&string);
    let mut mem_free:f64 = mem_free.parse::<f64>().expect("Err");
    mem_free = mem_free/1024.0/1024.0;
    mem_free = round_to_nth_digit(mem_free, 4);
    */

    // same
    string.clear();
    string.push_str("MemAvailable");
    let mem_available: String = find_memory_entry(memory_contents.to_string(), &string);
    let mut mem_available: f64 = mem_available.parse::<f64>().expect("Err");
    mem_available = mem_available / 1024.0 / 1024.0;
    mem_available = round_to_nth_digit(mem_available, 4);

    // Uptime
    // Open linux Uptime Info file
    let uptime_file = File::open("/proc/uptime");
    // create String to push content to
    let mut uptime_contents = String::new();
    // push content to created string
    match uptime_file
        .expect("REASON")
        .read_to_string(&mut uptime_contents)
    {
        Ok(_) => {}
        Err(_) => {
            println!("Error reading file")
        }
    }

    let uptime_contents_location = match uptime_contents.find(char::is_whitespace) {
        Some(pos) => pos,
        None => return,
    };

    uptime_contents = uptime_contents[..uptime_contents_location].to_string();
    let mut uptime_contents_float = uptime_contents.parse::<f64>().expect("Err");
    uptime_contents_float = uptime_contents_float / 60.0;

    let uptime_contents_str = if uptime_contents_float > 60.0 {
        uptime_contents_float = uptime_contents_float / 60.0;
        uptime_contents_float = round_to_nth_digit(uptime_contents_float, 2);
        format!("{} h", uptime_contents_float)
    } else {
        uptime_contents_float = round_to_nth_digit(uptime_contents_float, 2);
        format!("{} m", uptime_contents_float)
    };

    // Kernel Info
    let kernel_info_file = File::open("/proc/sys/kernel/osrelease");
    // create String to push content to
    let mut kernel_info_content = String::new();
    // push content to created string
    match kernel_info_file
        .expect("REASON")
        .read_to_string(&mut kernel_info_content)
    {
        Ok(_) => {}
        Err(_) => {
            println!("Error reading file")
        }
    }

    kernel_info_content = kernel_info_content.replace("\n", "").replace("\r", "");

    // User name
    /*  let mut user_name_file = File::open("/var/run/utmp").expect("Err");
    let mut buffer = Vec::new();

    user_name_file.read_to_end(&mut buffer).expect("Err"); // Read entire file as bytes

    let binary_string: String = buffer
        .iter()
        .map(|byte| format!("{:08b}", byte))
        .collect();


    let binding = convert(&binary_string);

    let re = Regex::new(r"ts[^A-Za-z]*([A-Za-z]+):").unwrap();


    let mut username: Option<String> = None;

    for cap in re.captures_iter(binding.as_str()) {
        username = Some(cap[1].to_string());
    }
    */

    let username = Command::new("/bin/bash")
        .arg("-c")
        .arg("whoami")
        .output()
        .expect("Err");
    let username = String::from_utf8_lossy(&username.stdout).trim().to_string();

    let distro_content = fs::read_to_string("/etc/os-release");

    let distro_content = match distro_content {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    let mut distro_name = String::new();

    if let Some(line) = distro_content.lines().find(|l| l.starts_with("NAME=")) {
        distro_name = line
            .trim_start_matches("NAME=")
            .trim_matches('"')
            .to_string();
    }

    // Disk Space
    let disks = Disks::new_with_refreshed_list();

    let total_space: u64 = disks[0].total_space() / 1024 / 1024 / 1024;
    let available_space: u64 = disks[0].available_space() / 1024 / 1024 / 1024;
    let used_space: u64 = total_space - available_space;

    // Cpu Name Model Name
    let cpu_model_name = Command::new("/bin/bash")
        .arg("-c")
        .arg("grep 'model name' /proc/cpuinfo | uniq")
        .output()
        .expect("Err");

    let cpu_model_name = String::from_utf8_lossy(&cpu_model_name.stdout)
        .trim()
        .to_string()
        .replace("model name", "")
        .replace(":", "")
        .trim()
        .to_string();

    /*    // packages
        let output = Command::new("rpm")
            .args(["-qa"])
            .output()
            .expect("failed to execute rpm");

       let package_count =  String::from_utf8_lossy(&output.stdout)
           .lines()
           .count();
    */

    /*
        print!("⠀⠀⠀⠀⠀⠀⠀⠀
        ⣀⣴⣦⡄⠀⠀⠀⠀⠀⠀⠀
⠀⠀⢠⢺⡽⣦⣴⣾⡿⣟⣿⢿⣷⣤⣺⠿⡳⡀⠀ Username: {}
⠀⠀⠸⣿⣠⣿⡿⠛⠛⢿⡟⠛⠻⣿⣷⣠⡷⠁⠀ Distro: {}
⠀⠀⠀⠀⠹⣿⠇⣠⡄⠀⠀⣠⡀⢹⣿⠉⣠⣤⣦ Kernel: {}
⠀⠀⠀⠀⠀⢿⡇⠀⠀⢂⠂⠀⠀⢿⡇⢸⣿⠋⠁ Uptime: {}
⠀⠀⠀⠀⠀⠘⣧⣈⠓⠚⠒⠋⣠⣞⠀⠘⢿⣷⡄ Memory: {}/{} GiB
⠀⠀⠀⠀⠀⢸⣿⡿⣿⣿⣿⣿⢿⣿⡆⠀⢀⣿⡷ Space: {}/{} GiB
⠀⠀⠀⣴⣶⣿⡿⣽⠟⠉⠉⢻⣿⢾⣷⣶⣿⠟⠁
⠀⣴⠋⢻⣿⣿⣟⣿⡀⠀⠀⢸⣿⢿⣿⣟⡟⢶⣄
⠈⢯⡀⠈⠻⢿⣿⡽⣇⠀⢀⡸⢿⣿⡿⠋⠀⣀⠝
⠀⠀⠙⠢⠴⠭⠤⠤⠬⠧⠼⠤⠤⠤⠽⠦⠖⠁⠀\n",
                 username.unwrap(), distro_name, kernel_info_content, uptime_contents_str,
                 round_to_nth_digit(mem_total-mem_available,2),round_to_nth_digit(mem_total,2),available_spcae,total_spcae);
    }
    */

    let ascii_art = vec![
        "        ⣀⣴⣦⡄",
        "⠀⠀⢠⢺⡽⣦⣴⣾⡿⣟⣿⢿⣷⣤⣺⠿⡳⡀",
        "⠀⠀⠸⣿⣠⣿⡿⠛⠛⢿⡟⠛⠻⣿⣷⣠⡷⠁",
        "⠀⠀⠀⠀⠹⣿⠇⣠⡄⠀⠀⣠⡀⢹⣿⠉⣠⣤⣦",
        "⠀⠀⠀⠀⠀⢿⡇⠀⠀⢂⠂⠀⠀⢿⡇⢸⣿⠋⠁",
        "⠀⠀⠀⠀⠀⠘⣧⣈⠓⠚⠒⠋⣠⣞⠀⠘⢿⣷⡄",
        "⠀⠀⠀⠀⠀⢸⣿⡿⣿⣿⣿⣿⢿⣿⡆⠀⢀⣿⡷",
        "⠀⠀⠀⣴⣶⣿⡿⣽⠟⠉⠉⢻⣿⢾⣷⣶⣿⠟⠁",
        "⠀⣴⠋⢻⣿⣿⣟⣿⡀⠀⠀⢸⣿⢿⣿⣟⡟⢶⣄",
        "⠈⢯⡀⠈⠻⢿⣿⡽⣇⠀⢀⡸⢿⣿⡿⠋⠀⣀⠝",
        "⠀⠀⠙⠢⠴⠭⠤⠤⠬⠧⠼⠤⠤⠤⠽⠦⠖",
    ];

    // System info variables

    // Build info lines from variables
    let info_lines = vec![
        "┌──────────────────────────────┐".to_string(),
        format!("Username: {}", username),
        format!("Distro: {}", distro_name),
        format!("Kernel: {}", kernel_info_content),
        format!("Cpu: {}", cpu_model_name),
        format!("Uptime: {}", uptime_contents_str),
        format!(
            "Memory: {}/{} GiB",
            round_to_nth_digit(mem_total - mem_available, 2),
            round_to_nth_digit(mem_total, 2)
        ),
        format!("Available space: {}/{} GiB", used_space, total_space),
        "└──────────────────────────────┘".to_string(),
    ];

    let max_width = ascii_art
        .iter()
        .map(|s| s.chars().count())
        .max()
        .unwrap_or(0);

    let total_lines = ascii_art.len().max(info_lines.len());
    let info_start = (total_lines - info_lines.len()) / 2;

    for i in 0..total_lines {
        let left = ascii_art.get(i).unwrap_or(&"");
        let right = if i >= info_start && i < info_start + info_lines.len() {
            &info_lines[i - info_start]
        } else {
            ""
        };

        println!("{:<width$}   {}", left, right, width = max_width);
    }
}
