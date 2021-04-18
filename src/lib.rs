use rquickjs::{Runtime, Context, Ctx, Func, bind, Class, Function, ClassDef, Object, Module, Error};

use crate::js_class::JSClass;
use crate::user::pop::Person;
use crate::js_class::animal::Dog;


pub mod user;

pub mod js_class;


pub struct JsEngine {
    pub rt: Runtime,
    pub ctx: Context,
}


impl JsEngine {
    // 支持自定义初始化
    pub fn custom_init(rt: Runtime, ctx: Context) -> JsEngine {
        JsEngine {
            rt,
            ctx,
        }
    }

    fn print(msg: String) {
        println!("{}", msg);
    }


    pub fn init() -> JsEngine {
        let rt = Runtime::new().unwrap();
        let ctx = Context::full(&rt).unwrap();
        ctx.with(|ctx| {
            let global = ctx.globals();
            // global.set("print", Func::new("print", JsEngine::print)).unwrap();
        });
        JsEngine {
            rt,
            ctx,
        }
    }

    pub fn add_object_proto(&self) {
        self.ctx.with(|ctx| {
            let global = ctx.globals();
            let object = Object::new(ctx).unwrap();
            object.set("square", "960");
            object.set("language", "chinese");
            global.set("china", object).unwrap();
        });
    }

    pub fn with<'js, F, R>(&self, func: F) -> R
        where
            F: FnOnce(Ctx) -> R,
    {
        self.ctx.with(func)
    }


    // 添加通用函数  绑定到object 随便调用 不需要new对象
    /* pub fn add_object_proto(&self) {
         self.ctx.with(|ctx| {
             let global = ctx.globals();
             global.set("print", Func::new("print", JsEngine::print)).unwrap();
         });
     }*/

    /*
        pub fn add_class(&self) {
            self.ctx.with(|ctx| {
                let global = ctx.globals();
                Class::<Dog>::register(ctx).unwrap();
                global.set(Dog::CLASS_NAME, Func::from(Class::<Dog>::constructor(Dog::new))).unwrap();
            });
        }*/


    pub fn run_js<'js, F, R>(&self, func: F) -> R where F: FnOnce(Ctx) -> R, {
        self.ctx.with(func)
    }

    fn call_js_function(&self, js_source: &str) {
        return self.ctx.with(|ctx| {
            let module: Module = ctx.compile("test", js_source).unwrap();
            let func: Function = module.get("hello").unwrap();
            let r = func.call::<_, ()>(());
            func.call::<_, ()>(()).unwrap();
        });
    }
}


/// the module must be add follow:
///   #[bind(object,public)]  and  #[quickjs(bare)]
#[macro_export]
macro_rules! register_module {
    ($a:expr, $( $x:ty ),* ) => {
        {
            $a.ctx.with(|ctx|
            {
                let global = ctx.globals();
                      $(
                       global.init_def::<$x>().unwrap();
                       )*
            });
        }
    };
}




/// the class must be impl classDef  or  class_def!  struct
/// must be add new fn, it is constructor method
/// must be set CLASS_NAME , you can  var o=new CLASS_NAME()  from js;
#[macro_export]
macro_rules! register_class {
    ($a:expr, $( $x:ty ),* ) => {
        {
            $a.ctx.with(|ctx|
            {
                let global = ctx.globals();
                      $(
                       Class::<$x>::register(ctx).unwrap();
                       global.set(<$x>::CLASS_NAME, Func::from(Class::<$x>::constructor(<$x>::new))).unwrap();
                       )*
            });
        }
    };
}

// like k-v    v: basic type or object
#[macro_export]
macro_rules! add_global_attribute {
    ($a:expr,$key:expr,$value:expr) => {
        {
              $a.ctx.with(|ctx|{
                let global = ctx.globals();
                 global.set($key, $value);
               })
        }
    };
}

// k-v    v:fn
#[macro_export]
macro_rules! add_global_fn {
    ($a:expr,$key:expr,$value:expr) => {
        {
              $a.ctx.with(|ctx|{
                let global = ctx.globals();
                 global.set($key, Func::new($key, $value)).unwrap();
               })
        }
    };
}


/// the class must be impl classDef  or  class_def!  struct
/// must be add new fn, it is constructor method
/// must be set CLASS_NAME , you can  var o=new CLASS_NAME()  from js;
#[macro_export]
macro_rules! call_js_function {
    ($a:expr,$source:expr,$name:expr, $( $x:expr ),* ) => {
        {
          $a.ctx.with(|ctx|
            {
                 let module: Module = ctx.compile("test", $source).unwrap();
                 let func: Function = module.get("echo").expect("555");
                   let rr=($($x),*);
                  let rra:String=func.call(rr).unwrap();
                 println!("aaa  {:?}",rra);
            });
          //  println!("aaa  {:?}",r);

        }
    };
}
// $($arg)*)

/*pub fn call_js_function(&self, js_source: &str) {
    return self.ctx.with(|ctx| {
        let module: Module = ctx.compile("test", js_source).unwrap();
        let func: Function = module.get("hello").unwrap();
        func.call::<_, ()>(()).unwrap();
    });
}*/


/// 适合自定义
pub fn init_js_engine() -> (Runtime, Context) {
    let rt = Runtime::new().unwrap();
    let ctx = Context::full(&rt).unwrap();
    ctx.with(|ctx| {
        let global = ctx.globals();
        global.set("print", Func::new("print", JsEngine::print)).unwrap();
    });
    (rt, ctx)
}

/// 快捷调用 一次性完事
pub fn test_with<'js, F, R>(func: F) -> R
    where
        F: FnOnce(Ctx) -> R,
{
    let rt = Runtime::new().unwrap();
    let ctx = Context::full(&rt).unwrap();
    ctx.with(func)
}



