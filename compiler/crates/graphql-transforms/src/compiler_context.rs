/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use graphql_ir::{ExecutableDefinition, FragmentDefinition, OperationDefinition};
use interner::StringKey;
use schema::Schema;
use std::collections::HashMap;
use std::sync::Arc;

/// A collection of all documents that are being compiled.
#[derive(Debug)]
pub struct CompilerContext<'s> {
    schema: &'s Schema,
    fragments: HashMap<StringKey, Arc<FragmentDefinition>>,
    operations: Vec<Arc<OperationDefinition>>,
}

impl<'s> CompilerContext<'s> {
    pub fn new(schema: &'s Schema) -> Self {
        Self {
            schema,
            fragments: Default::default(),
            operations: Default::default(),
        }
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn from_definitions(schema: &'s Schema, definitions: Vec<ExecutableDefinition>) -> Self {
        let mut operations = Vec::new();
        let mut fragments = HashMap::new();
        for definition in definitions {
            match definition {
                ExecutableDefinition::Operation(operation) => {
                    operations.push(Arc::new(operation));
                }
                ExecutableDefinition::Fragment(fragment) => {
                    fragments.insert(fragment.name.item, Arc::new(fragment));
                }
            }
        }
        Self {
            schema,
            fragments,
            operations,
        }
    }

    pub fn insert_fragment(&mut self, fragment: Arc<FragmentDefinition>) {
        let name = fragment.name.item;
        if let Some(previous) = self.fragments.insert(name, fragment) {
            panic!(
                "Can only insert '{}' once. Had {:?} and trying to insert {:?}.",
                name, previous, self.fragments[&name]
            );
        };
    }

    pub fn fragment(&self, name: StringKey) -> Option<&Arc<FragmentDefinition>> {
        self.fragments.get(&name)
    }

    pub fn insert_operation(&mut self, operation: Arc<OperationDefinition>) {
        self.operations.push(operation);
    }

    pub fn operations(&self) -> impl Iterator<Item = &Arc<OperationDefinition>> {
        self.operations.iter()
    }

    pub fn fragments(&self) -> impl Iterator<Item = &Arc<FragmentDefinition>> {
        self.fragments.values()
    }

    pub fn document_count(&self) -> usize {
        self.fragments.len() + self.operations.len()
    }
}
