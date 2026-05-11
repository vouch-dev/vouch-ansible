use thirdpass_ansible_lib;
use thirdpass_core::extension::FromLib;

fn main() {
    let mut extension = thirdpass_ansible_lib::AnsibleExtension::new();
    thirdpass_core::extension::commands::run(&mut extension).unwrap();
}
