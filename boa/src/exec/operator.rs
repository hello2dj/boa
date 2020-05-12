//! Operator execution.

use super::{Executable, Interpreter};
use crate::{
    builtins::value::{ResultValue, Value},
    environment::lexical_environment::VariableScope,
    syntax::ast::{
        node::{Assign, BinOp, Node},
        op::{self, AssignOp, BitOp, CompOp, LogOp, NumOp},
    },
};

impl Executable for Assign {
    fn run(&self, interpreter: &mut Interpreter) -> ResultValue {
        let val = interpreter.exec(self.rhs())?;
        match self.lhs() {
            Node::Local(ref name) => {
                if interpreter.realm().environment.has_binding(name.as_ref()) {
                    // Binding already exists
                    interpreter.realm_mut().environment.set_mutable_binding(
                        name.as_ref(),
                        val.clone(),
                        true,
                    );
                } else {
                    interpreter.realm_mut().environment.create_mutable_binding(
                        name.as_ref().to_owned(),
                        true,
                        VariableScope::Function,
                    );
                    interpreter
                        .realm_mut()
                        .environment
                        .initialize_binding(name.as_ref(), val.clone());
                }
            }
            Node::GetConstField(ref obj, ref field) => {
                let val_obj = interpreter.exec(obj)?;
                val_obj.set_field_slice(&field.clone(), val.clone());
            }
            Node::GetField(ref obj, ref field) => {
                let val_obj = interpreter.exec(obj)?;
                let val_field = interpreter.exec(field)?;
                val_obj.set_field(val_field, val.clone());
            }
            _ => (),
        }
        Ok(val)
    }
}

impl Executable for BinOp {
    fn run(&self, interpreter: &mut Interpreter) -> ResultValue {
        match self.op() {
            op::BinOp::Num(op) => {
                let v_a = interpreter.exec(self.lhs())?;
                let v_b = interpreter.exec(self.rhs())?;
                Ok(match op {
                    NumOp::Add => v_a + v_b,
                    NumOp::Sub => v_a - v_b,
                    NumOp::Mul => v_a * v_b,
                    NumOp::Exp => v_a.as_num_to_power(v_b),
                    NumOp::Div => v_a / v_b,
                    NumOp::Mod => v_a % v_b,
                })
            }
            op::BinOp::Bit(op) => {
                let v_a = interpreter.exec(self.lhs())?;
                let v_b = interpreter.exec(self.rhs())?;
                Ok(match op {
                    BitOp::And => v_a & v_b,
                    BitOp::Or => v_a | v_b,
                    BitOp::Xor => v_a ^ v_b,
                    BitOp::Shl => v_a << v_b,
                    BitOp::Shr => v_a >> v_b,
                    // TODO Fix
                    BitOp::UShr => v_a >> v_b,
                })
            }
            op::BinOp::Comp(op) => {
                let mut v_a = interpreter.exec(self.lhs())?;
                let mut v_b = interpreter.exec(self.rhs())?;
                Ok(Value::from(match op {
                    CompOp::Equal if v_a.is_object() => v_a == v_b,
                    CompOp::Equal => v_a == v_b,
                    CompOp::NotEqual if v_a.is_object() => v_a != v_b,
                    CompOp::NotEqual => v_a != v_b,
                    CompOp::StrictEqual if v_a.is_object() => v_a == v_b,
                    CompOp::StrictEqual => v_a == v_b,
                    CompOp::StrictNotEqual if v_a.is_object() => v_a != v_b,
                    CompOp::StrictNotEqual => v_a != v_b,
                    CompOp::GreaterThan => v_a.to_number() > v_b.to_number(),
                    CompOp::GreaterThanOrEqual => v_a.to_number() >= v_b.to_number(),
                    CompOp::LessThan => v_a.to_number() < v_b.to_number(),
                    CompOp::LessThanOrEqual => v_a.to_number() <= v_b.to_number(),
                    CompOp::In => {
                        if !v_b.is_object() {
                            panic!("TypeError: {} is not an Object.", v_b);
                        }
                        let key = interpreter.to_property_key(&mut v_a);
                        interpreter.has_property(&mut v_b, &key)
                    }
                }))
            }
            op::BinOp::Log(op) => {
                // turn a `Value` into a `bool`
                let to_bool = |value| bool::from(&value);
                Ok(match op {
                    LogOp::And => Value::from(
                        to_bool(interpreter.exec(self.lhs())?)
                            && to_bool(interpreter.exec(self.rhs())?),
                    ),
                    LogOp::Or => Value::from(
                        to_bool(interpreter.exec(self.lhs())?)
                            || to_bool(interpreter.exec(self.rhs())?),
                    ),
                })
            }
            op::BinOp::Assign(op) => match self.lhs() {
                Node::Local(ref name) => {
                    let v_a = interpreter
                        .realm()
                        .environment
                        .get_binding_value(name.as_ref());
                    let v_b = interpreter.exec(self.rhs())?;
                    let value = Self::run_assign(op, v_a, v_b);
                    interpreter.realm.environment.set_mutable_binding(
                        name.as_ref(),
                        value.clone(),
                        true,
                    );
                    Ok(value)
                }
                Node::GetConstField(ref obj, ref field) => {
                    let v_r_a = interpreter.exec(obj)?;
                    let v_a = v_r_a.get_field_slice(field);
                    let v_b = interpreter.exec(self.rhs())?;
                    let value = Self::run_assign(op, v_a, v_b);
                    v_r_a.set_field_slice(&field.clone(), value.clone());
                    Ok(value)
                }
                _ => Ok(Value::undefined()),
            },
        }
    }
}

impl BinOp {
    fn run_assign(op: AssignOp, v_a: Value, v_b: Value) -> Value {
        match op {
            AssignOp::Add => v_a + v_b,
            AssignOp::Sub => v_a - v_b,
            AssignOp::Mul => v_a * v_b,
            AssignOp::Exp => v_a.as_num_to_power(v_b),
            AssignOp::Div => v_a / v_b,
            AssignOp::Mod => v_a % v_b,
            AssignOp::And => v_a & v_b,
            AssignOp::Or => v_a | v_b,
            AssignOp::Xor => v_a ^ v_b,
            AssignOp::Shl => v_a << v_b,
            AssignOp::Shr => v_a << v_b,
        }
    }
}
