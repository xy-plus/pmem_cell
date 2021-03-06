pub mod utils;

use lazy_static::*;
use spin::Mutex;
use std::alloc::{dealloc, Layout};
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::ptr;
pub use utils::*;

#[repr(C)]
#[derive(Clone)]
pub struct PMemCell<T> {
    addr: *mut T,
}

impl<T> fmt::Debug for PMemCell<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("PMemCell")
                .field("addr", &self.addr)
                .field("src", &Box::leak(Box::from_raw(self.addr)))
                .finish()
        }
    }
}

impl<T> Default for PMemCell<T>
where
    T: Default + Clone + PMemTrans,
{
    fn default() -> Self {
        return Self::new();
    }
}

impl<T> Drop for PMemCell<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.addr);
            dealloc(self.addr as *mut u8, Layout::new::<T>());
        }
    }
}

impl<T> PMemCell<T>
where
    T: Default + Clone + PMemTrans,
{
    pub fn new() -> Self {
        // TODO: alloc NVM instead of T::default()
        let src = Box::new(T::default());
        let ret = PMemCell {
            addr: Box::into_raw(src),
        };
        ret
    }
}

// TODO: use NVM instead of Box
impl<T> CrashSafe<T> for PMemCell<T>
where
    T: PMemTrans + Clone,
{
    fn get(&mut self) -> &mut T {
        unsafe { Box::leak(Box::from_raw(self.addr)) }
    }
    // TODO: read member
    unsafe fn get_member(&mut self, name: &str) -> &mut pm_u64 {
        let index = T::name_to_index(name);
        Box::leak(Box::from_raw(
            (self.addr as *mut u64).add(index as usize) as *mut pm_u64
        ))
    }
    // TODO: persistent_write full struct
    fn persistent_write(&mut self, val: &T) {
        unsafe {
            *Box::leak(Box::from_raw(self.addr)) = val.clone();
        }
    }
}

lazy_static! {
    static ref __TRANS_TABLE: Mutex<HashMap<String, HashMap<String, usize>>> =
        Mutex::new(HashMap::new());
}

pub fn print_table() {
    println!("------- TRANS_TABLE -------");
    println!("{:#?}", __TRANS_TABLE.lock().iter());
    println!("---------------------------");
}

pub fn trans_table_insert(class_name: String, k2: String, v: usize) {
    __TRANS_TABLE
        .lock()
        .get_mut(&class_name)
        .unwrap()
        .insert(k2, v);
}

pub fn new_struct_table(k1: String) {
    __TRANS_TABLE.lock().insert(k1, HashMap::new());
}

pub fn trans_table_name_to_index(class_name: String, member_name: &str) -> usize {
    return *__TRANS_TABLE
        .lock()
        .get_mut(&class_name)
        .unwrap()
        .get(member_name)
        .unwrap();
}

pub fn struct_is_init(class_name: String) -> bool {
    match __TRANS_TABLE.lock().get(&class_name) {
        None => return false,
        _ => return true,
    }
}

#[macro_export]
macro_rules! pmem_cell_def_struct {
    ($name:ident { $($member_name:ident : $member_type:ty),* }) => {
        #[repr(C)]
        #[derive(Debug, Clone, Default)]
        struct $name {
            // TODO: all type use 64bit len, only support u64 currently
            $($member_name : $member_type),*
        }

        impl pmem_cell::PMemTrans for $name {
            fn init() {
                // println!("{}", stringify!($($member_name : $member_type),*));
                pmem_cell::new_struct_table(stringify!($name).to_string());
                for (i, x) in stringify!($($member_name : $member_type),*).split(',').enumerate() {
                    let mut x_iter = x.split(':');
                    let x = x_iter.next().unwrap().trim().to_string();
                    pmem_cell::trans_table_insert(stringify!($name).to_string(), x, i as usize);
                    let x = x_iter.next().unwrap().trim().to_string();
                    assert!(pmem_cell::utils::type_is_pmemcell(x));
                }
            }
            fn name_to_index(name: &str) -> usize {
                if !pmem_cell::struct_is_init(name.to_string()) {
                    $name::init();
                }
                return pmem_cell::trans_table_name_to_index(stringify!($name).to_string(), name);
            }
        }
    };
}

pub unsafe fn trans_type<T>(pm_u64_: &mut pm_u64) -> &mut PMemCell<T> {
    return &mut *(pm_u64_ as *mut pm_u64 as *mut _ as *mut PMemCell<T>); //as &mut PMemCell<T>;
}
