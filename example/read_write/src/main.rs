use pmem_cell::{
    print_table, trans_type,
    utils::{pm_u64, CrashSafe, PMemTrans},
    PMemCell,
};

#[derive(Debug, Clone, Default)]
struct TestStruct {
    a: u64,
    b: u64,
}

impl PMemTrans for TestStruct {
    fn init() {}
    fn name_to_index(_name: &str) -> usize {
        return 0;
    }
}

#[macro_use]
extern crate pmem_cell;

pmem_cell_def_struct!(Test1 {
    a: pm_u64,
    b: pm_u64
});
pmem_cell_def_struct!(Test2 {
    c: pm_u64,
    d: PMemCell<Test1>
});
pmem_cell_def_struct!(Test3 {
    e: PMemCell<TestStruct>,
    f: pm_u64
});

fn main() {
    println!("------- Test 1 -------");
    let mut a = PMemCell::<Test1>::new();
    println!("{:#?}", a.get());
    unsafe {
        let t1 = a.get_member("b");
        t1.persistent_write(&233);
    }
    println!("{:#?}", a.get());
    println!("------- Test 2 -------");
    let mut b = PMemCell::<Test2>::new();
    println!("{:#?}", b.get());
    unsafe {
        let t2 = trans_type::<Test1>(b.get_member("d"));
        t2.persistent_write(a.get());
    }
    println!("{:#?}", b.get());
    println!("------- Test 3 -------");
    let mut c = PMemCell::<Test3>::new();
    println!("{:#?}", c.get());
    unsafe {
        let t3 = trans_type::<TestStruct>(c.get_member("e"));
        t3.persistent_write(&TestStruct { a: 77, b: 88 });
    }
    println!("{:#?}", c.get());
    println!("----------------------");
    print_table();
}
