use vouch_ansible_lib;
use vouch_lib::extension::FromLib;

fn main() {
    let mut extension = vouch_ansible_lib::AnsibleExtension::new();
    vouch_lib::extension::commands::run(&mut extension).unwrap();
}
