/*#[macro_export]
macro_rules! add_class {
($ctx:expr,$($x:expr),*) => {{


})
}}
}
*/

use std::fmt::DebugStruct;
#[macro_export]
macro_rules! ppp {
    ($a:expr, $( $x:expr ),* ) => {
        {
        println!("{}",$a.get());
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


struct Pj;

impl Pj {
    pub fn to_string() {
        print!("hello Pj");
    }
    pub fn get(&self) -> String {
        String::from("get string")
    }
}

fn main() {
    //  let r = ppp!();
    let pj = Pj {};
    let a = ppp!(pj,"555","5555");
    println!("{:?}", a)
}