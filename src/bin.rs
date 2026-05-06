use thirdpass_ansible_lib;
use thirdpass_lib::extension::FromLib;

fn main() {
    let mut extension = thirdpass_ansible_lib::AnsibleExtension::new();
    thirdpass_lib::extension::commands::run(&mut extension).unwrap();
}
