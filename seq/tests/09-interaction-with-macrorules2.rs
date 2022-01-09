//! 对照组：如果不使用 `09-interaction-with-macrorules.rs` 的技巧，那么无法成功生成 PROCS
//!
//! 因为声明宏生成的常量不一定在过程宏展开时被知晓（也可能那个常量根本还未生成）。

use seq::seq;

macro_rules! literal_identity_macro {
    () => {
        256
    };
}

// Expands to: `const NPROC: usize = 256;`
const NPROC: usize = literal_identity_macro!();

struct Proc;

impl Proc {
    const fn new() -> Self { Proc }
}

// Expands to: `static PROCS: [Proc; NPROC] = [Proc::new(), ..., Proc::new()];`
static PROCS: [Proc; NPROC] = seq!(N in 0..NPROC { [#(Proc::new(),)*] });

fn main() {}
