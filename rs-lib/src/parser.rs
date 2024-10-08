// Copyright 2018-2024 the Deno authors. MIT license.

use anyhow::Result;
use deno_ast::parse_module;
use deno_ast::ParseDiagnostic;
use deno_ast::ParseParams;
use deno_ast::ParsedSource;
use deno_graph::ModuleParser;
use deno_graph::ParseOptions;

#[derive(Default, Copy, Clone)]
pub struct ScopeAnalysisParser;

impl ModuleParser for ScopeAnalysisParser {
  fn parse_module(
    &self,
    options: ParseOptions,
  ) -> Result<ParsedSource, ParseDiagnostic> {
    parse_module(ParseParams {
      specifier: options.specifier.clone(),
      text: options.source,
      media_type: options.media_type,
      capture_tokens: true,
      scope_analysis: true,
      maybe_syntax: None,
    })
  }
}
