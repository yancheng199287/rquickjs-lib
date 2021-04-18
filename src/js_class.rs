use rquickjs::{ObjectDef, Ctx, Object, Func, class_def, Method, ClassDef};

pub trait JSClass: ClassDef {
    // 构造方法
    // 打印当前对象的各个属性
    fn to_json_string(&self);
}


pub mod animal {
    use crate::js_class::JSClass;
    use rquickjs::{ObjectDef, Ctx, Result, Object, Func, class_def, Method, ClassDef, RefsMarker, Value, ClassId, Class};

    #[derive(Clone)]
    pub struct Dog {}
    class_def!(
            Dog
            (proto) {
                println!("X::register");
                proto.set("getAge", Func::from(Method(Dog::get_age)))?;
            }
            @(ctor) {
                ctor.set("getName", Func::from(Dog::get_name))?;
            }
        );

    impl Dog {
        pub const CLASS_NAME: &'static str = "Dog";

        pub fn new() -> Self {
            println!("A::new {}", "dog");
            Self {}
        }

        pub fn get_age(&self) -> u32 {
            1699
        }


        pub fn get_name() -> String {
            "small Black".to_string()
        }
    }


    #[derive(Clone)]
    pub struct AirPlane {}
    class_def!(
            AirPlane
            (proto) {
                println!("X::register");
                proto.set("getAge", Func::from(Method(AirPlane::get_age)))?;
            }
            @(ctor) {
                ctor.set("getName", Func::from(AirPlane::get_name))?;
            }
        );


    impl AirPlane {
        pub const CLASS_NAME: &'static str = "AirPlane";

        pub fn new() -> Self {
            println!("A::new {}", "AirPlane");
            Self {}
        }

        pub fn get_age(&self) -> u32 {
            1699
        }


        pub fn get_name() -> String {
            "small AirPlane".to_string()
        }
    }
}