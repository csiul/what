extern crate model;
extern crate config;
extern crate tools;

use model::target::{Target, WPSState};
use tools::tshark::Tshark;
use config::Configuration;

use std::process::{Command, Stdio};
use std::collections::HashSet;
use std::time::SystemTime;

struct Airodump {
    // The name of the interface to use
    interface: String,
    // The list of targets (access points and clients) being tracked
    targets: Vec<Target>,
    // The wireless channel to scan (if specified)
    channel: Option<u8>,
    // Whether the interface is a 5 GHz interface
    five_ghz: bool,
    // The encryption type to filter for (if specified)
    encryption: Option<String>,
    // The WPS state to filter for
    wps: WPSState,
    // The BSSID of the target access point (if specified)
    target_bssid: Option<String>,
    // The prefix to use for the output files
    output_file_prefix: String,
    // Whether to only include IVs in the output
    ivs_only: bool,
    // Whether to skip WPS-enabled access points
    skip_wps: bool,
    // Whether to delete existing temp files before starting airodump
    delete_existing_files: bool,
    // The prefix to use for the CSV output files
    csv_file_prefix: String,
    // The process ID of the airodump process (if it is running)
    pid: Option<std::process::Child>,
    // Whether decloaking (revealing previously hidden access points) is enabled
    decloaking: bool,
    // The set of BSSIDs of decloaked access points
    decloaked_bssids: HashSet<String>,
    // A map of BSSIDs to the times they were last deauthenticated
    decloaked_times: std::collections::HashMap<String, SystemTime>,
}

impl Airodump {
    fn new(
        interface: Option<String>,
        channel: Option<u8>,
        encryption: Option<String>,
        wps: WPSState,
        target_bssid: Option<String>,
        output_file_prefix: String,
        ivs_only: bool,
        skip_wps: bool,
        delete_existing_files: bool,
    ) -> Airodump {
        // Get the interface name from the configuration if it wasn't specified
        let interface = interface.unwrap_or(Configuration::interface().unwrap());
        let five_ghz = Configuration::five_ghz();
        let channel = channel.unwrap_or(Configuration::target_channel());
        // Create the prefix for the CSV output files
        let csv_file_prefix = format!("{}{}", Configuration::temp(), output_file_prefix);
        Airodump {
            interface,
            targets: Vec::new(),
            channel,
            five_ghz,
            encryption,
            wps,
            target_bssid,
            output_file_prefix,
            ivs_only,
            skip_wps,
            delete_existing_files,
            csv_file_prefix,
            pid: None,
            decloaking: false,
            decloaked_bssids: HashSet::new(),
            decloaked_times: std::collections::HashMap::new(),
        }
    }

    // Start the airodump process with the specified arguments
    fn start_airodump(&mut self) {
         // Delete existing temp files if necessary
        if self.delete_existing_files {
            self.delete_airodump_temp_files(&self.output_file_prefix);
        }

        // Build the command to start airodump
        let mut command = Command::new("airodump-ng");
        command.arg(&self.interface)
            .arg("-a") // Only show associated clients
            .arg("-w") // Output file prefix
            .arg(&self.csv_file_prefix)
            .arg("--write-interval") // Write every second
            .arg("1");

        if let Some(channel) = self.channel {
            command.arg("-c").arg(channel.to_string()); // Use the specified channel
        } else if self.five_ghz {
            command.arg("--band").arg("a"); // Use the 5 GHz band
        }

        if let Some(encryption) = &self.encryption {
            command.arg("--enc").arg(encryption); // Filter by encryption type
        }

       
        if self.wps == WPSState::KNOWN {
            command.arg("--wps"); // Filter by WPS state
        }

        if let Some(bssid) = &self.target_bssid {
            command.arg("--bssid").arg(bssid); // Filter by BSSID
        }

        // Only include IVs in the output if necessary
        if self.ivs_only {
            command.arg("--output-format").arg("ivs,csv");
        } else {
            command.arg("--output-format").arg("pcap,csv");
        }

        // Start the airodump process
        let child = command.stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to start airodump");
        self.pid = Some(child);
    }

    // Search for files with the specified file extension in the specified directory
    fn find_files(&self, endswith: &str) -> Vec<String> {
        self.find_files_by_output_file_prefix(endswith, &self.output_file_prefix)
    }

    // Update the list of targets (access points and clients) being tracked
    fn update_targets(&mut self) {
        // Find all CSV files with the specified output file prefix
        let files = self.find_files(".csv");
        for file in files {
             // Parse the targets from the CSV file using Tshark
            let tshark = Tshark::new(file, self.interface.clone());
            let mut new_targets = tshark.parse_targets();
            self.targets.append(&mut new_targets);
        }
    }

    // Delete temp files created by airodump with the specified output file prefix
    fn delete_airodump_temp_files(&self, output_file_prefix: &str) {
        let files = self.find_files_by_output_file_prefix("*", output_file_prefix);
        for file in files {
            std::fs::remove_file(file).expect("failed to delete airodump temp file");
        }
    }

    // Search for files with the specified output file prefix and file extension
    fn find_files_by_output_file_prefix(&self, endswith: &str, output_file_prefix: &str) -> Vec<String> {
        let mut results = Vec::new();
        let temp_dir = Configuration::temp();
        let path = std::path::Path::new(&temp_dir);
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().expect("invalid Unicode in filename");
                if file_name.starts_with(output_file_prefix) && file_name.ends_with(endswith) {
                    results.push(format!("{}{}", temp_dir, file_name));
                }
            }
        }
        results
    }
}

impl Drop for Airodump {
    // Perform cleanup when the Airodump object goes out of scope
    fn drop(&mut self) {
        if let Some(pid) = self.pid.take() {
            // Interrupt the airodump process
            pid.kill().expect("failed to kill airodump process");
        }

        if self.delete_existing_files {
            // Delete temp files if necessary
            self.delete_airodump_temp_files(&self.output_file_prefix);
        }
    }
}
