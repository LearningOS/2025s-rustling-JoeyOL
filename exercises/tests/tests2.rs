// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.



// #cfg[(test)]是一个条件编译属性，用于标记模块或者代码库啊仅在运行测试时编译和执行
#[cfg(test)]
mod tests {
    // 当运行cargo test时，附有#[test]的函数会被自动执行
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(1, 1);
    }
}
