// Bitfield enums with any discriminant (implicit or explicit) outside of the
// range 0..2^BITS should fail to compile.

use bitfield::*;

const F: isize = 1;

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

#[test]
#[should_panic]
fn enum_check_bits() {
    // 运行时检查枚举体的 discriminant 是否越界（原 09 测试是编译时检查的，而且可定位到具体的成员上）
    // dtolnay 对 bitfield 的具体实现没有过多介绍，有些 failed 类型的测试很难通过
    DeliveryMode::__check_bits(); 
}
