

pub mod Devices {

    
    pub fn get_devices() -> String{

        use std::process::{Command, Stdio};

        let com = Command::new("ls")
                        .arg("/sys/class/net")
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("failed to execute child");


        let result = com
                        .wait_with_output()
                        .expect("failed to wait on child");
                        
        let output = String::from_utf8(result.stdout).unwrap();


        return output;
    }


    pub fn get_wifi_devices() -> Vec<String>{
        
        let devices_str = get_devices();
        let all: Vec<&str> = devices_str.split("\n").collect();
        let mut selected: Vec<String> =  Vec::new();

        for s in all {

            if s.len() > 0 && s.chars().nth(0).unwrap() == 'w' {

                selected.push(String::from(s));
            }
        }


        return selected;
    }


    pub fn get_phy(wlan_name: &str) -> String{

        use std::process::{Command, Stdio};


        let com = Command::new("iw")
                        .arg("dev")
                        .arg(wlan_name)
                        .arg("info")
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("failed to execute child");


        let result = com
                        .wait_with_output()
                        .expect("failed to wait on child");
                        

        let output = String::from_utf8(result.stdout).unwrap();
        let mut phy_name: String = ("").to_string();
        let pos = output.find("wiphy");


        if pos != None {

            let start = pos.unwrap() + 6;
            let mut end = start +1;

            while &output[end..(end+1)] != "\n" {
                end += 1;
            }

            phy_name = (&output[start..end]).to_string();
        }


        return format!("{}{}", "phy", phy_name);
    }


    pub fn monitor_mode_compatible(phy_name: &str) -> bool{

        use std::process::{Command, Stdio};


        let com = Command::new("iw")
                        .arg("phy")
                        .arg(phy_name)
                        .arg("info")
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("failed to execute child");


        let result = com
                        .wait_with_output()
                        .expect("failed to wait on child");
                        
        let output = String::from_utf8(result.stdout).unwrap();
        let pos = output.find("* monitor");

        if pos != None {

            return true;
        }



        return false;
    }



    pub fn list_all_devices_monitor_mode_compatible() -> Vec<String>{

        
        let devs:Vec<String> = get_wifi_devices();
        let mut devs_mon_mode:Vec<String> =  Vec::new();


        for dev in devs {

            let phy = get_phy(&dev);
            
            if monitor_mode_compatible(&phy) {
                println!("{}",dev);
                devs_mon_mode.push(dev);
            } 
        }


        return devs_mon_mode;
    }
}
