use rquickjs::{bind, Result};


#[bind(object,public)]
#[quickjs(bare)]
pub mod user {
    pub struct UserService {}

    impl UserService {
        pub fn new() -> Self {
            UserService {}
        }
        pub fn hello(&self) {
            println!("UserService ok");
        }
    }
}


pub mod pop {
    use rquickjs::{ObjectDef, Ctx, Object, Func, class_def, Method};

    pub struct Person {
        pub x: f64,
        pub y: f64,
    }

    impl Person {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn zero() -> Self {
            Self::new(0.0, 0.0)
        }

        pub fn get_x(&self) -> f64 {
            self.x
        }
    }

    class_def! {
          // 类的实例原型方法
            Person (proto) {
                proto.set("get_x", Func::from(Method(Person::get_x)))?;
                proto.set("get_y", Func::from(Method(|Person { y, .. }: &Person| *y)))?;
            }
            // 静态方法
            @(ctor) {
                ctor.set("zero", Func::from(Person::zero))?;
            }
        }

    /* pub struct Person {}
     class_def!(Person);

     impl Person {
         pub fn new() -> Self {
             Person {}
         }
         pub fn hello(&self) {
             println!("ok");
         }
     }*/


    pub struct CliObject {}

    impl ObjectDef for CliObject {
        fn init<'js>(ctx: Ctx<'js>, object: &Object<'js>) -> rquickjs::Result<()> {
            println!("load person object");

            object.set("name", "yancheng");

            object.set("cat", Func::from(|a: String, b: String| format!("{}{}", a, b))).unwrap();
            Result::Ok(())
        }
    }
}


