// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

//结构体分为三种类型：
/**
 *  
 * 1.经典的 C 语言风格结构体（C struct）
 *      1.1 先定义
 *          struct ColorClassicStruct {
                red:u64,
                green:u64,
                blue:u64,
            }
        
        1.2 再创建实例
            let mut green = ColorClassicStruct {
                red:0,
                green:255,
                blue:0,
            };

        green.red = 1;  改变值
 * 2.元组结构体（tuple struct），事实上就是具名元组而已。
 *      当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，
 * 元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了。
 *      2.1 先定义 
 *              struct ColorTupleStruct(u64,u64,u64);
 *      2.2 再创建实例 
 *              let green = ColorTupleStruct(0,255,0);
 * 
   3.单元结构体（unit struct），不带字段，在泛型中很有用。
    类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
 *      3.1 先定义 
 *              struct UnitLikeStruct;
 *      3.2 再创建实例 
 *              let unit_like_struct = UnitLikeStruct;
 */


struct ColorClassicStruct {
    // TODO: Something goes here
    red:u64,
    green:u64,
    blue:u64,
}

struct ColorTupleStruct(u64,u64,u64);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            red:0,
            green:255,
            blue:0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);

        
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(0,255,0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
