
pub mod dependency;

static dependency_required:bool = true;
static dependency_name:&str = "iw";
static dependency_url:&str = "sudo apt-get install iw";


struct Devices{
    dependency:Dependency,
}