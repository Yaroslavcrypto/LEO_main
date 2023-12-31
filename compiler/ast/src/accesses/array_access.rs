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

use crate::{Expression, Node};
use leo_span::Span;

use std::fmt;

use serde::{Deserialize, Serialize};

/// An array element access expression `array[index]`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayAccess {
    /// The expression, evaluating to an array, that is being indexed.
    pub array: Box<Expression>,
    /// The index in `array` that is being accessed.
    pub index: Box<Expression>,
    /// The span of the entire expression `array[index]`.
    pub span: Span,
}

impl fmt::Display for ArrayAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}]", self.array, self.index)
    }
}

impl Node for ArrayAccess {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
