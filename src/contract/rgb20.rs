use rgbstd::interface::rgb20;
use rgbstd::interface::{Amount, ContractID, Fungible, Divisible};
use rgbstd::vm::{SubSchema, RgbIsa};
use rgb_schemata::{nia_rgb20, nia_schema};

impl rgb20::interface {
    // Add your implementations here
  fn nia_rgb20() -> IfaceImpl {
    let schema = nia_schema();
    let iface = rgb20();
    let aluvm = aluvm();
        
   fn IfaceImpl = {
      let version = V1
      let schema_id = schema_id();
      let iface_id = iface_id();
      let script = let script();
      let global_state = let global_state();
      let nominal = let nominal();
      let data = let data();
      let created = let created();
      let issued_suply = let issued_supply();
}

impl rgb20::schemata {
    // Add your implementations here
    fn schemata() - IfaveImpl {
    let schema = schema();

    fn schemata = {
    let schema = let schema();
    let subschema  = let subschema();
    let alu_id  = let alu_id();
    
}

