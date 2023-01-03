pub mod dependency;
pub mod process;

use std::collections::HashSet;

static dependency_required:bool = true;
static dependency_name:&str = "iwconfig";
static dependency_url:&str = "apt-get install wireless-tools";

struct Iwconfig{
    dependency:Dependency,
}

impl Iwconfig{
    fn mode(&self, iface:String, mode_name:String) -> String{
        process::Process();
        let args = vec!["iwconfig".to_string(), iface, "mode".to_string(), mode_name];
        let pid = Process(args);
        pid.wait();
        return pid.poll();
    }

    fn get_interfaces(&self, mode:Option<String>) -> Vec<String>{
        process::Process();
        let mut interfaces = HashSet::new();
        let mut iface = String::new();
        let (out, err) = Process.call("iwconfig".to_string());
        for line in out.lines(){
            if line.len() == 0 {
                continue
            }
            if !(line.starts_with(" ")){
                    iface = line.split_whitespace()[0];
                    if iface.contains("\t"){
                        iface = iface.split('\t').collect()[0].trim();
                    }
                    iface = iface.replace(" ", "");
                    if iface.len() == 0 {
                        continue
                    }
                    if mode == None{
                        interfaces.insert(iface);
                    }
            }
            let cond = format!("Mode:{mode}");
            if mode != None && line.contains(cond) && iface.len() > 0{
                interfaces.insert(iface);
            }
        }
        return Vec::from_iter(interfaces);
    }
}