#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PMemCell<u64>(u64);

#[allow(non_camel_case_types)]
pub type pm_u64 = PMemCell<u64>;

pub trait PMemTrans {
    fn init();
    fn name_to_index(name: &str) -> usize;
}

pub trait CrashSafe<T> {
    fn get(&mut self) -> &mut T;
    unsafe fn get_member(&mut self, name: &str) -> &mut pm_u64;
    fn persistent_write(&mut self, val: &T);
}

// TODO: use NVM to make it persistent
// now it is fake
impl CrashSafe<u64> for pm_u64 {
    fn get(&mut self) -> &mut u64 {
        return &mut self.0;
    }
    unsafe fn get_member(&mut self, _name: &str) -> &mut pm_u64 {
        unimplemented!()
    }
    fn persistent_write(&mut self, val: &u64) {
        self.0 = *val;
    }
}

impl PMemTrans for pm_u64 {
    fn init() {}
    fn name_to_index(_name: &str) -> usize {
        unimplemented!()
    }
}

pub fn type_is_pmemcell(s: String) -> bool {
    if s == "pm_u64" {
        return true;
    }
    let v: Vec<_> = s.match_indices("PMemCell<").collect();
    if v.len() == 0 {
        return false;
    }
    return v[0] == (0, "PMemCell<");
}
