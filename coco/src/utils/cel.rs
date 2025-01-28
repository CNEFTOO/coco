use cel_interpreter::{Context, Program, Value, ExecutionError};
use md5;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub trait Handler {
    fn call(&self, args: &[Value]) -> Result<Value, ExecutionError>;
}

pub fn add_function<H: Handler + Send + Sync + 'static>(context: &mut Context, name: &str, _handler: H) {
    context.add_variable(name, Value::Function(std::sync::Arc::new(name.to_string()), None));
}

pub fn create_context() -> Context<'static> {
    let mut context = Context::default();
    register_function(&mut context);
    context
}

pub fn run_program(exp: &str) {
    let mut context = create_context();
    register_function(&mut context);
    let program = Program::compile(exp).unwrap();
    program.execute(&context).unwrap();
}

pub fn register_function(context: &mut Context) {
    #[derive(Clone)]
    struct Md5Handler;
    unsafe impl Send for Md5Handler {}
    unsafe impl Sync for Md5Handler {}
    impl Handler for Md5Handler {
        fn call(&self, args: &[Value]) -> Result<Value, ExecutionError> {
            if let Some(Value::String(s)) = args.first() {
                Ok(Value::String(md5_hash(s.to_string()).into()))
            } else {
                Ok(Value::String(String::new().into()))
            }
        }
    }
    add_function(context, "md5", Md5Handler);

    #[derive(Clone)]
    struct B64Handler;
    unsafe impl Send for B64Handler {}
    unsafe impl Sync for B64Handler {}
    impl Handler for B64Handler {
        fn call(&self, args: &[Value]) -> Result<Value, ExecutionError> {
            if let Some(Value::String(s)) = args.first() {
                Ok(Value::String(base64encoding(s.to_string()).into()))
            } else {
                Ok(Value::String(String::new().into()))
            }
        }
    }
    add_function(context, "b64", B64Handler);

    #[derive(Clone)]
    struct D64Handler;
    unsafe impl Send for D64Handler {}
    unsafe impl Sync for D64Handler {}
    impl Handler for D64Handler {
        fn call(&self, args: &[Value]) -> Result<Value, ExecutionError> {
            if let Some(Value::String(s)) = args.first() {
                Ok(Value::String(base64decoding(s.to_string()).into()))
            } else {
                Ok(Value::String(String::new().into()))
            }
        }
    }
    add_function(context, "d64", D64Handler);
}

fn md5_hash(input: String) -> String {
    format!("{:x}", md5::compute(input.as_bytes()))
}

fn base64encoding(input: String) -> String {
    BASE64.encode(input.as_bytes())
}

fn base64decoding(input: String) -> String {
    match BASE64.decode(input) {
        Ok(bytes) => String::from_utf8_lossy(bytes.as_slice()).to_string(),
        Err(_) => String::from("Invalid base64 input")
    }
}