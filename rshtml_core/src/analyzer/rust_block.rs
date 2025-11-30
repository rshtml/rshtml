use crate::{analyzer::Analyzer, position::Position};
use anyhow::{Result, anyhow};
use syn::{Block, Pat, Stmt, parse_str};

pub struct RustBlockAnalyzer;

impl RustBlockAnalyzer {
    pub fn analyze(analyzer: &mut Analyzer, content: &str, _position: &Position) -> Result<()> {
        let component_name = if let Some(name) = &analyzer.is_component {
            name.to_string()
        } else {
            return Ok(());
        };

        let content = if content.trim().starts_with('{') {
            content
        } else {
            &format!("{{ {} }}", content)
        };

        let block: Block = match parse_str(&content) {
            Ok(x) => x,
            Err(_) => {
                return Ok(());
            }
        };

        let mut variables = Vec::new();

        for stmt in block.stmts {
            if let Stmt::Local(local) = stmt {
                Self::extract_idents_from_pat(&local.pat, &mut variables);
            }
        }

        let component = analyzer
            .components
            .get(&component_name)
            .ok_or(anyhow!("Couldn't find component {component_name}"))?;

        variables.retain(|x| !component.parameters.contains(x));

        analyzer
            .components
            .entry(component_name)
            .and_modify(|component| component.code_block_vars.append(&mut variables));

        Ok(())
    }

    fn extract_idents_from_pat(pat: &Pat, vars: &mut Vec<String>) {
        match pat {
            Pat::Ident(pat_ident) => {
                let name = pat_ident.ident.to_string();
                if name != "_" {
                    vars.push(name);
                }
            }
            Pat::Type(pat_type) => {
                Self::extract_idents_from_pat(&pat_type.pat, vars);
            }
            Pat::Tuple(pat_tuple) => {
                for p in &pat_tuple.elems {
                    Self::extract_idents_from_pat(p, vars);
                }
            }
            Pat::Struct(pat_struct) => {
                for field in &pat_struct.fields {
                    Self::extract_idents_from_pat(&field.pat, vars);
                }
            }
            Pat::TupleStruct(pat_tuple_struct) => {
                for p in &pat_tuple_struct.elems {
                    Self::extract_idents_from_pat(p, vars);
                }
            }
            Pat::Reference(pat_ref) => {
                Self::extract_idents_from_pat(&pat_ref.pat, vars);
            }
            Pat::Slice(pat_slice) => {
                for p in &pat_slice.elems {
                    Self::extract_idents_from_pat(p, vars);
                }
            }
            _ => {}
        }
    }
}
