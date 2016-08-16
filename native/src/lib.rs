#[macro_use]
extern crate neon;

use neon::mem::Handle;
use neon::vm::Lock;
use neon::js::{JsString, JsInteger, JsFunction, Object, Value};
use neon::js::class::{JsClass, Class};

pub struct Greeter {
    greeting: String
}

impl Drop for Greeter {
    fn drop(&mut self) {
        println!("dropping greeter: {}", self.greeting);
    }
}

declare_types! {

    /// A simple native class for creating greeting strings.
    pub class JsGreeter for Greeter {
        // This is invoked when the constructor is called with `new`, before the
        // `constructor` operation is invoked.
        init(call) {
            let scope = call.scope;
            println!("extracting the greeting parameter");
            let greeting = try!(try!(call.arguments.require(scope, 0)).to_string(scope)).value();
            println!("extracted the greeting parameter");
            Ok(Greeter {
                greeting: greeting
            })
        }

        // This is invoked when the constructor is called as a function.
        call(call) {
            println!("in construct.[[Call]]");
            Ok(JsInteger::new(call.scope, 3).upcast())
        }

        // This is invoked when the constructor is called with `new`, after the `init`
        // operation has constructed the internal `Greeter` data structure.
        constructor(call) {
            let scope = call.scope;
            let greeting = call.arguments.this(scope).grab(|greeter| {
                greeter.greeting.clone()
            });
            println!("in constructor.[[Construct]], greeting is {}", greeting);

            // JS constructors can use the obscure "constructor override pattern"
            // to return a different value than the `this` object, which ends up
            // being the result of the `new` expression. If you want to do that,
            // you can return Some(override_value). Normally you just return None.
            Ok(None)
        }

        method hello(call) {
            let scope = call.scope;
            let name = try!(try!(call.arguments.require(scope, 0)).to_string(scope)).value();
            let msg = call.arguments.this(scope).grab(|greeter| {
                format!("{}, {}!", greeter.greeting, name)
            });
            Ok(try!(JsString::new_or_throw(scope, &msg[..])).upcast())
        }
    }

    /// A simple demonstration of a class whose constructor can only be invoked with `new`
    /// syntax, not called as a function.
    pub class JsUncallable as Uncallable for () {
        init(_) {
            Ok(())
        }

        method snarf(call) {
            println!("snarf.");
            Ok(JsInteger::new(call.scope, 42).upcast())
        }
    }
}

register_module!(m, {
    let class: Handle<JsClass<JsGreeter>> = try!(JsGreeter::class(m.scope));
    let constructor: Handle<JsFunction<JsGreeter>> = try!(class.constructor(m.scope));
    try!(m.exports.set("Greeter", constructor));

    let class: Handle<JsClass<JsUncallable>> = try!(JsUncallable::class(m.scope));
    let constructor: Handle<JsFunction<JsUncallable>> = try!(class.constructor(m.scope));
    try!(m.exports.set("Uncallable", constructor));

    Ok(())
});
