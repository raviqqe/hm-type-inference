use crate::ast::*;
use crate::types::Type;
use std::collections::HashMap;

#[derive(Debug)]
pub struct InferenceError;

pub fn infer(
    environment: &HashMap<String, Type>,
    expression: &Expression,
) -> Result<(HashMap<usize, Type>, Type), InferenceError> {
    Ok(match expression {
        Expression::Application(function, argument) => {
            let (mut substitutions, function_type) = infer(&environment, &function)?;
            let (other_substitutions, argument_type) = infer(&environment, &argument)?;

            substitutions.extend(other_substitutions);

            let result_type = Type::new_variable();

            substitutions.extend(unify(
                &function_type,
                &Type::Function(argument_type.into(), result_type.clone().into()),
            )?);

            let result_type = result_type.substitute(&substitutions);

            (substitutions, result_type)
        }
        Expression::Lambda(variable, expression) => {
            let argument_type = Type::new_variable();

            let mut environment = environment.clone();
            environment.insert(variable.clone(), argument_type.clone());

            let (substitutions, result_type) = infer(&environment, &expression)?;
            let function_type =
                Type::Function(argument_type.into(), result_type.into()).substitute(&substitutions);

            (substitutions, function_type)
        }
        Expression::Let(variable, bound_expression, expression) => {
            let (mut substitutions, type_) = infer(&environment, &bound_expression)?;

            let mut environment = environment.clone();
            environment.insert(variable.clone(), type_.clone());

            let (other_substitutions, type_) = infer(&environment, &expression)?;

            substitutions.extend(other_substitutions);

            (substitutions.clone(), type_.substitute(&substitutions))
        }
        Expression::Number => (Default::default(), Type::Number),
        Expression::Variable(variable) => (
            Default::default(),
            environment.get(variable).ok_or(InferenceError)?.clone(),
        ),
    })
}

fn unify(one: &Type, other: &Type) -> Result<HashMap<usize, Type>, InferenceError> {
    Ok(match (one, other) {
        (Type::Variable(variable), other) | (other, Type::Variable(variable)) => {
            vec![(variable.clone(), other.clone())]
                .into_iter()
                .collect()
        }
        (Type::Number, Type::Number) => Default::default(),
        (
            Type::Function(one_argument, one_result),
            Type::Function(other_argument, other_result),
        ) => {
            let mut substitutions = unify(one_argument, other_argument)?;

            substitutions.extend(unify(one_result, other_result)?);

            substitutions
        }
        _ => return Err(InferenceError),
    })
}
