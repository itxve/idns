use sysdns::SysDNS;

pub struct IDNS;

impl IDNS {
  pub fn set_dns(server: String) -> () {
    let _ = SysDNS {
      enable: SysDNS::is_support(),
      server,
    }
    .set_system_dns();
  }

  pub fn get_system_dns() -> String {
    let sysdns = SysDNS::get_system_dns().unwrap();
    sysdns.server
  }

  pub fn empty_dns() -> () {
    IDNS::set_dns("Empty".to_owned())
  }
}
