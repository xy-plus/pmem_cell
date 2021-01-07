# 崩溃一致性 RFC

引入一个 trait `CrashSafe`，语义是：具有此特性的类型能够在崩溃后，恢复一致的数据。

```rust
trait CrashSafe {
  /// 提交当前更改，函数返回时保证生效
  fn commit(&mut self);
  /// 恢复到最后一次 commit 时的数据
  fn recover(&mut self);
}
```

举例说明：

```rust
let x: &mut Pmem<(u32, u32)> = &mut NVM 上的整数对 (0,0);
x.0 = 1;		// p1
x.1 = 2;		// p2
x.commit();	// p3

// 程序在某p处崩溃，重启恢复
x.recover();
assert_eq!(*x, (0,0));	// 在 p1 p2 处崩溃
assert_eq!(*x, (1,2));	// 在 p3 处崩溃
```

对任意数据类型 T，有一种基于 shadow copy 的 naive 实现：

```rust
// 将 T 扩展为以下结构
struct Safe<T> {
  /// 表示当前修改的是哪一个变量，另一个保持修改前的值
  /// 在任意时刻保证：另一份数据是干净的
  dirty: bool,
  v: [T; 2],
}
impl<T> Safe<T> {
  fn new(v: T) {
    Safe {
      dirty: false,
      v: [v, v],
    }
  }
}
// 修改时访问脏数据
impl DerefMut for Safe<T> {
  type Target = T;
  fn deref_mut(&mut self) -> &mut T {
    self.v[self.dirty]
  }
}
impl<T> CrashSafe for Safe<T> {
  fn commit(&mut self) {
    persist(&self.v[self.dirty]);
    self.dirty = !self.dirty;
    persist(&self.dirty);	// commit point：dirty位翻转落盘
    self.v[self.dirty] = self.v[!self.dirty];
  }
  fn recover(&mut self) {
    self.v[self.dirty] = self.v[!self.dirty];
  }
}
```

接下来考虑如何组合。。。