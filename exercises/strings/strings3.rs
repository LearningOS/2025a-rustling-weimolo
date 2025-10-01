// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

//只有String::new()和String::from()才是创建String类型的正确方式，其他双引号括起来的都是&str类型
//push_str 方法:向字符串末尾添加一个字符串切片（&str）,直接将整个字符串切片的内容追加到原字符串，不会获取参数的所有权
//push 方法:向字符串末尾添加单个字符（char 类型）,一次只能添加一个字符，支持所有 Unicode 字符（包括中文、emoji 等）
//chars() 是 str 类型的一个方法:用于返回字符串的字符迭代器（Chars 类型）(chars() 是 str 类型的一个方法，用于返回字符串的字符迭代器（Chars 类型）)
//replace 方法:replace 会对原始字符串进行扫描和替换，最终将结果收集到一个新分配的 String 中并返回,返回的是 String 类型
//trim() 方法: 移除字符串两端空白字符,返回的是 &str 类型,trim 会创建一个新的字符串切片，指向原始字符串中去除空白字符后的部分,不会分配新的内存空间


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut result = String::new();
    /* 
    ("  What's up!"), "What's up!")
    
    
    
    for c in input.chars(){
        if c!=' '{
            result.push(c);
        }
    }
    result
    */

    result = input.trim().to_string();
    result
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut result = String::from(input);
    result.push_str(" world!");
    result
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let result = input.replace("cars", "balloons");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
