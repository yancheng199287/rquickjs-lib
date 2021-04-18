use rquickjs_lib::JsEngine;


fn main() {
    let js_engine=JsEngine::init();
    js_engine.with(|ctx|{
       let a:String= ctx.eval(r#"
          var name="yancheng";
          console.log("Hello",name);
          aa
        "#).unwrap();
    })
}