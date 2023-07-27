// 任何单元测试不通过，整个集成测试都不会通过
// 二进制 crate 如果只包含 src/main.rs, 那么只能包含集成测试, 不能包含单元测试

use tests;

// common 模块用于辅助测试
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, tests::add_two(2));
}
