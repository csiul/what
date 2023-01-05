extern crate dependency;
extern crate config;
extern crate util;
extern crate tools;

use dependency::Dependency;
use config::Configuration;
use util::color::Color;
use util::process::Process;
use tools::hashcat::HcxPcapTool;

use std::fs;
use std::process::Command;

struct John;

impl John {
    fn crack_handshake(handshake: &str, show_command: bool) -> String {
        let john_file = HcxPcapTool::generate_john_file(handshake, show_command);

        let formats_stdout = Process::new(vec!["john", "--list=formats"]).stdout();
        let john_format = if formats_stdout.contains("wpapsk-opencl") {
            "wpapsk-opencl"
        } else if formats_stdout.contains("wpapsk-cuda") {
            "wpapsk-cuda"
        } else {
            "wpapsk"
        };

        let command = vec![
            "john",
            &format!("--format={}", john_format),
            "--wordlist", &Configuration::wordlist,
            &john_file,
        ];

        if show_command {
            Color::pl(&format!("{{+}} {{D}}Running: {{W}}{{P}}{}{{W}}", command.join(" ")));
        }
        let process = Process::new(command);
        process.wait();

        let command = vec!["john", "--show", &john_file];
        if show_command {
            Color::pl(&format!("{{+}} {{D}}Running: {{W}}{{P}}{}{{W}}", command.join(" ")));
        }
        let process = Process::new(command);
        let stdout = process.get_output().stdout;

        if stdout.contains("0 password hashes cracked") {
            return "".to_string();
        } else {
            for line in stdout.split('\n') {
                if line.contains(handshake.capfile) {
                    return line.split(':').collect::<Vec<_>>()[1].to_string();
                }
            }
        }

        fs::remove_file(john_file).expect("Unable to delete file");

        "".to_string()
    }
}
