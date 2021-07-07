// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::errors::ImportParserError;
use leo_asg::{AsgContext, AsgConvertError, ImportResolver, Program, Span};

use indexmap::{IndexMap, IndexSet};
use std::{collections::HashMap, path::PathBuf};

/// Stores imported packages.
///
/// A program can import one or more packages. A package can be found locally in the source
/// directory, foreign in the imports directory, or part of the core package list.
#[derive(Clone, Default)]
pub struct ImportParser<'a> {
    program_path: PathBuf,
    partial_imports: IndexSet<String>,
    imports: IndexMap<String, Program<'a>>,
    pub imports_map: HashMap<String, String>,
}

impl<'a> ImportParser<'a> {
    pub fn new(program_path: PathBuf, imports_map: HashMap<String, String>) -> Self {
        ImportParser {
            program_path,
            partial_imports: Default::default(),
            imports: Default::default(),
            imports_map,
        }
    }
}

impl<'a> ImportResolver<'a> for ImportParser<'a> {
    fn resolve_package(
        &mut self,
        context: AsgContext<'a>,
        package_segments: &[&str],
        span: &Span,
    ) -> Result<Option<Program<'a>>, AsgConvertError> {
        let full_path = package_segments.join(".");
        if self.partial_imports.contains(&full_path) {
            return Err(ImportParserError::recursive_imports(&full_path, span).into());
        }
        if let Some(program) = self.imports.get(&full_path) {
            return Ok(Some(program.clone()));
        }
        let path = self.program_path.clone();
        let mut imports = self.clone(); // Self::default() was previously
        self.partial_imports.insert(full_path.clone());
        let program = imports
            .parse_package(context, path, package_segments, span)
            .map_err(|x| -> AsgConvertError { x.into() })?;
        self.partial_imports.remove(&full_path);
        self.imports.insert(full_path, program.clone());
        Ok(Some(program))
    }
}
