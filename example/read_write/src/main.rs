use pmem_cell::{
    print_table,
    utils::{pm_u64, CrashSafe, PMemTrans},
    PMemCell,
};

#[macro_use]
extern crate pmem_cell;

pmem_cell_def_struct!(Test1 {
    a: pm_u64,
    b: pm_u64
});
pmem_cell_def_struct!(Test2 {
    c: pm_u64,
    b: pm_u64
});

fn main() {
    Test1::init();
    Test2::init();
    print_table();
    let mut a = PMemCell::<Test1>::new();
    println!("{:#?}", a.get());
    let t1 = a.get_member("b");
    t1.persistent_write(233);
    println!("{:#?}", a.get());
}
