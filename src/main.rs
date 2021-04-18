use rquickjs_lib::{JsEngine, init_js_engine};
use rquickjs::{Runtime, Context, Object, bind, ObjectDef, ModuleDef, Class, Function, Func, Module, Result, This};
use rquickjs_lib::user::pop::{Person, CliObject};
use rquickjs_lib::js_class::animal::{Dog, AirPlane};
use rquickjs_lib::{register_module, register_class, add_global_attribute, add_global_fn, call_js_function};

mod user;

/*#[bind(object)]
#[quickjs(bare)]
pub mod losa {

    use rquickjs::Value;

    pub struct Log {}

    impl Log {
        pub fn new() -> Self {
            Log {}
        }
        pub fn log(&self, values: Vec<Value>) {
            for value in values {
                print!("values{:?}", value.as_string());
            }
        }
    }
}*/

pub mod my1 {
    use rquickjs::Value;

    pub struct Log {}

    impl Log {
        pub fn new() -> Self {
            Log {}
        }
        pub fn log(&self, values: Vec<Value>) {
            for value in values {
                print!("values{:?}", value.as_string());
            }
        }
    }
}

#[bind(object)]
#[quickjs(bare)]
mod geom {
    pub struct Point {}

    impl Point {
        // constructor
        pub fn new() -> Self {
            Point {}
        }
        pub fn ok(&self) {
            println!("ok");
        }
    }
}


use std::fmt::DebugStruct;

pub fn print(msg: String) {
    println!("{}", msg);
}

fn main() {

    // let (rt, ctx) = init_js_engine();
    let js_engine = JsEngine::init();

    //js_engine.add_class();

    register_module!(js_engine, user::User,Geom);
    register_class!(js_engine, Dog,AirPlane);
    add_global_fn!(js_engine,"print",print);
    add_global_attribute!(js_engine,"yun",850);


    js_engine.add_object_proto();
    js_engine.with(|ctx| {
        let glob = ctx.globals();
        glob.init_def::<CliObject>().unwrap();
        // glob.init_def::<Geom>().unwrap();

        // 注意外部模块的路径   先要在本模块 声明 mod user;  或者 use 使用哪个文件的模块
        // 这种方法不需要写很多初始化参数
        //  glob.init_def::<user::User>().unwrap();

        /*     Class::<Person>::register(ctx).unwrap();
             let ctor = Function::new(ctx, Class::<Person>::constructor(Person::new)).unwrap();
             glob.set("Person", ctor).unwrap();*/
    });

    let r = js_engine.with(|ctx| {
        let glob = ctx.globals();
        Class::<Person>::register(ctx).unwrap();
        let ctor = Function::new(ctx, Class::<Person>::constructor(Person::new)).unwrap();
        glob.set("Person", ctor).unwrap();


        let res: f32 = ctx.eval(r#"
        var user=new UserService();
        user.hello();
          var person=new Person(15,20);
           print("person get x "+person.get_x());
             print("person  get y "+person.get_y());
          var a=cat("123","456");
          print("==>"+a)
         var point=new Point();
        point.ok();
        var dog=new Dog();

        print("dog age:"+dog.getAge());
        print("dog name:"+Dog.getName());

          var airPlane=new AirPlane();
          print("airPlane age:"+airPlane.getAge());
        print("airPlane name:"+AirPlane.getName());

           print("china name:"+china.language);

              print("china name:"+china);
                 print("china name:"+yun);

        55
        "#).unwrap();
        println!("{}", res);

        let module: Module = ctx.compile("test", r#"
           export function hello(){
                  var airPlane=new AirPlane();
                  print("airPlane age:"+airPlane.getAge());
              }

           export function echo(args){
                  print("your input args:"+args);
                  return args;
              }
        "#, ).unwrap();

        let names = module.names().collect::<Result<Vec<String>>>().unwrap();

        for name in names {
            print!("name=> {}", name)
        }

        let func: Function = module.get("hello").unwrap();
        func.call::<_, ()>(()).unwrap();


        let obj = Object::new(ctx).unwrap();
        let func1: Function = module.get("echo").unwrap();
        let res: i32 = func1.call((303, )).unwrap();
        println!("call js function, result:{}", res);

        return String::from("result is ok");
        // let obj = Object::new(ctx).unwrap();
    });

    println!("eval result:{}", r);


    let source: &str = r#"
           export function hello(){
                  var airPlane=new AirPlane();
                  print("airPlane age:"+airPlane.getAge());
              }

           export function echo(args1,args2){
                  print("your input args   :"+args1+", "+args2);
                  return args1;
              }
        "#;
     call_js_function!(js_engine,source,"echo","ppp","hhhh");

       // println!("----- {:?}",opr);
    //let re: Option<String> = call_js_function!(js_engine,source,"hello","I am a clever pig");
    // println!("call_js_function result:{}", re.is_some())
}
