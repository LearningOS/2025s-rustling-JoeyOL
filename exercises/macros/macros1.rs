// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    // 模式匹配如果没有传入参数就执行这段语句
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
