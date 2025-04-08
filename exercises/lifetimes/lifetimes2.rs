// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.



// 'a 声明了一个声明周期'a，表示x和y的引用必须至少在声明周期'a内有效，
// 目前的作用就是要求x和y必须具有同样的生命周期
// 也可以使不同的参数有不同的声明周期，同时还可以用‘:’约束不同生命周期的大小
fn longest<'a:'b, 'b>(x: &'a str, y: &'b str) -> &'b str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }
}
