use cel_interpreter::{context::Context, Program};
pub fn create_context() -> Context {
    let context = Context::default();

    context
}

pub fn run_program(exp: &str) {
    let mut context = create_context();
    register_function(&mut context);
    let program = Program::compile(exp).unwrap();
    program.execute(&context).unwrap();
}

pub fn register_function(context: &mut Context) {
    context.add_function("md5", md5);
    context.add_function("b64", base64decoding);
    context.add_function("d64", base64decoding);
}

fn md5() -> String {
    "".to_string()
}

fn base64encoding() -> String {
    "".to_string()
}

fn base64decoding() -> String {
    "".to_string()
}