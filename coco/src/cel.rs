use cel_interpreter::{
    types::{MapType, StringType, BoolType, InType, BytesType, AnyType, NullType},
    context::Context,
    environment::{EnvOptions, Function, Overload},
    cel_types::CelType,
};

pub fn create_interpreter() -> cel_interpreter::environment::Interpreter {
    let str_str_map_type = MapType::new(Box::new(StringType), Box::new(StringType));

    let new_env_options = EnvOptions::new();

    let context = Context::new();
    new_env_options.into_interpreter(context)
}
