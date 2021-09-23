use super::{ComputedValue, Token};
use std::collections::HashMap;

pub struct Environment {
    variables: HashMap<String,ComputedValue>,
    parent_scope: Option<Box<Environment>>
}

impl Environment {
    pub fn new_root_environment() -> Environment {
        Environment { variables: HashMap::new(), parent_scope: None }
    }

    pub fn new_environment(parent_env: Box<Environment>) -> Environment {
        let parent_scope = Some(parent_env);
        Environment { variables: HashMap::new(), parent_scope }
    }

    pub fn get(&self, identifier: &str) -> Option<&ComputedValue> {
        if let Some(v) = self.variables.get(identifier) {
            Some(v)
        } else {
            if let Some(p) = &self.parent_scope {
                p.get(identifier)
            } else {
                None
            }
        }
    } 

    pub fn put(&mut self, identifier: &str, value: ComputedValue) -> () {
        self.variables.insert(identifier.to_string(),value);
    }

    pub fn assign(&mut self, identifier: &str, value: ComputedValue) -> super::Result<()> {
        if let Some(_) = self.get(identifier) {
            self.put(identifier,value);
            Ok(())
        } else {
            Err(super::InterpreterError::new("Attempted to assign value to undeclared identifier"))
        }
        
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_gets_variable_from_same_scope() {
        let mut env = super::Environment::new_root_environment();
        env.put("test", super::ComputedValue::NumberValue(2.1));
        let res = env.get("test").unwrap();
        assert_eq!(&super::ComputedValue::NumberValue(2.1), res);
    }

    #[test]
    fn it_gets_variable_from_parent_scope() {
        let mut env = super::Environment::new_root_environment();
        env.put("hello", super::ComputedValue::NumberValue(2.1));

        let mut child_env = super::Environment::new_environment(Box::from(env));
        let res = child_env.get("hello").unwrap();
        assert_eq!(&super::ComputedValue::NumberValue(2.1), res);
    }
}