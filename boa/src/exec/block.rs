use super::{Executable, Interpreter};
use crate::{
    builtins::value::{ResultValue, Value},
    environment::lexical_environment::new_declarative_environment,
    syntax::ast::node::Block,
};

impl Executable for Block {
    fn run(&self, interpreter: &mut Interpreter) -> ResultValue {
        {
            let env = &mut interpreter.realm_mut().environment;
            env.push(new_declarative_environment(Some(
                env.get_current_environment_ref().clone(),
            )));
        }

        let mut obj = Value::null();
        for statement in self.as_ref() {
            obj = interpreter.exec(statement)?;

            // early return
            if interpreter.is_return {
                break;
            }
        }

        // pop the block env
        let _ = interpreter.realm_mut().environment.pop();

        Ok(obj)
    }
}
