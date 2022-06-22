// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_ast::*;

use crate::{Declaration, Flattener};

// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::Value;

impl<'a> ExpressionReconstructor for Flattener<'a> {
    type AdditionalOutput = Option<Value>;
    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        // let x: u32 = 10u32;
        // let y: u32 = x;

        // return y == 10u32;

        /*
        function main(b: bool) {
            let x = 0;
            if b {
                x = 1;
            } else {
                x = 2;
            }
            x == 1
        }

        function main() {
            let x = 0;
            if true {
                x = 1;
            } else {
                x = 2;
            }
            x == 1
        }
        */
        match &self
            .symbol_table
            .borrow()
            .lookup_variable(input.name)
            .unwrap()
            .declaration
        {
            Declaration::Const(Some(c)) | Declaration::Mut(Some(c)) => {
                (Expression::Literal(c.clone().into()), Some(c.clone()))
            }
            _ => (Expression::Identifier(input), None),
        }
    }

    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Call(CallExpression {
                function: input.function,
                arguments: input
                    .arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg).0)
                    .collect(),
                span: input.span,
            }),
            None,
        )
    }

    fn reconstruct_binary(&mut self, input: BinaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (_, left_const_value) = self.reconstruct_expression(*input.left.clone());
        let (_, right_const_value) = self.reconstruct_expression(*input.right.clone());

        match (left_const_value, right_const_value) {
            (Some(left_value), Some(right_value)) if !left_value.is_int_type() && !right_value.is_int_type() => {
                (Expression::Binary(input), None)
            }
            (Some(left_value), Some(right_value)) => {
                let value = match &input.op {
                    BinaryOperation::Add => left_value.add(right_value, input.span),
                    BinaryOperation::AddWrapped => todo!(),
                    BinaryOperation::And => todo!(),
                    BinaryOperation::BitwiseAnd => todo!(),
                    BinaryOperation::Div => todo!(),
                    BinaryOperation::DivWrapped => todo!(),
                    BinaryOperation::Eq => left_value.eq(right_value, input.span),
                    BinaryOperation::Ge => todo!(),
                    BinaryOperation::Gt => todo!(),
                    BinaryOperation::Le => todo!(),
                    BinaryOperation::Lt => todo!(),
                    BinaryOperation::Mul => todo!(),
                    BinaryOperation::MulWrapped => todo!(),
                    BinaryOperation::Nand => todo!(),
                    BinaryOperation::Neq => todo!(),
                    BinaryOperation::Nor => todo!(),
                    BinaryOperation::Or => todo!(),
                    BinaryOperation::BitwiseOr => todo!(),
                    BinaryOperation::Pow => left_value.pow(right_value, input.span),
                    BinaryOperation::PowWrapped => todo!(),
                    BinaryOperation::Shl => todo!(),
                    BinaryOperation::ShlWrapped => todo!(),
                    BinaryOperation::Shr => todo!(),
                    BinaryOperation::ShrWrapped => todo!(),
                    BinaryOperation::Sub => todo!(),
                    BinaryOperation::SubWrapped => todo!(),
                    BinaryOperation::Xor => todo!(),
                };

                if let Err(err) = value {
                    self._handler.emit_err(err);
                    (Expression::Binary(input), None)
                } else {
                    let value = value.unwrap();
                    (Expression::Literal(value.clone().into()), Some(value))
                }
            }
            _ => (Expression::Binary(input), None),
        }
    }
}