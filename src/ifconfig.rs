use regex::Regex;

// Vérifier la façon d'importer
// Comme les autres modules/fichiers ne sont pas créés, il est difficile de s'assurer que tout fonctionne
pub mod dependency;
pub mod process;

static dependency_required:bool = true;
static dependency_name:&str = "ifconfig";
static dependency_url:&str = "apt-get install net-tools";


struct Ifconfig{
    dependency:Dependency,
}

impl Ifconfig {
    // cls n'est pas utilisé dans la fonction, utilité de la variable à vérifier
    fn up(cls: &str, interface: &str, args: &mut [&str]){
        // Put interface up
        process::Process();

        let mut command = vec!["ifconfig", interface];
        // À vérifier l'utilité du if
        // Code en python
        // if type(args) is list:
        //     command.extend(args)
        // elif type(args) is 'str':
        //     command.append(args)
        //command.append('up')
        command.extend_from_slice(args);
        command.push("up");

        let pid = Process(command);
        pid.wait();

        if pid.poll() != 0 {
            panic!("{}", format!("Error putting interface {} up:\n{}\n{}", interface, pid.stdout(), pid.stderr()));
        }
    }

    // cls n'est pas utilisé dans la fonction, utilité de la variable à vérifier
    fn down(cls: &str, interface: &str){
        // Put interface down
        process::Process();

        let pid = Process(["ifconfig", interface, "down"]);
        pid.wait();

        if pid.poll() != 0 {
            panic!("{}", format!("Error putting interface {} down:\n{}\n{}", interface, pid.stdout(), pid.stderr()));
        }
    }

    // cls n'est pas utilisé dans la fonction, utilité de la variable à vérifier
    fn get_mac(cls: &str, interface: &str) -> String{
        process::Process();

        let output = Process(["ifconfig", interface]).stdout();

        // Mac address separed by dashes
        let mac_dash_regex = Regex::new(r"^[a-zA-Z0-9]{2}-[a-zA-Z0-9]{2}-[a-zA-Z0-9]{2}-[a-zA-Z0-9]{2}-[a-zA-Z0-9]{2}-[a-zA-Z0-9]{2}").unwrap();
        let match_bool = mac_dash_regex.is_match(output);

        if match_bool {
            return output.replace("-", ":");
        }

        // Mac address separed by colons
        let mac_colon_regex = Regex::new(r"^[a-zA-Z0-9]{2}:[a-zA-Z0-9]{2}:[a-zA-Z0-9]{2}:[a-zA-Z0-9]{2}:[a-zA-Z0-9]{2}:[a-zA-Z0-9]{2}").unwrap();
        let match_bool = mac_colon_regex.is_match(output);

        if match_bool {
            return output.to_string();
        }

        panic!("Could not find the mac address for {}", interface);
    }
}
