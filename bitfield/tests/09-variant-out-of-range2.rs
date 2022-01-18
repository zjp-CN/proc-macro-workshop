// Bitfield enums with any discriminant (implicit or explicit) outside of the
// range 0..2^BITS should fail to compile.

use bitfield::*;

const F: isize = 1;
// const F: isize = 0;

#[derive(BitfieldSpecifier)]
enum DeliveryMode {
    Fixed = F,
    Lowest,
    SMI,
    RemoteRead,
    NMI,
    Init,
    Startup,
    External,
}

// 编译时检查枚举体的 discriminant 是否越界（原 09 test）
// dtolnay 对 bitfield  的具体实现没有过多介绍，有些 failed 类型的测试很难通过
fn main() { }
