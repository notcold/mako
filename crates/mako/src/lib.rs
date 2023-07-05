#![feature(box_patterns)]
#![feature(hasher_prefixfree_extras)]

mod analyze_deps;
// mod analyze_statement;
mod ast;
mod bfs;
mod build;
mod chunk;
mod chunk_graph;
mod cli;
pub mod compiler;
pub mod config;
mod config_node_polyfill;
mod copy;
mod css_modules;
mod defined_ident_collector;
pub mod dev;
mod generate;
mod generate_chunks;
mod group_chunk;
mod hmr;
mod load;
pub mod logger;
mod minify;
mod module;
mod module_graph;
mod parse;
mod resolve;
mod sourcemap;
// mod statement;
// mod statement_graph;
mod targets;
#[cfg(test)]
mod test_helper;
mod transform;
mod transform_css_handler;
mod transform_dep_replacer;
mod transform_dynamic_import;
mod transform_env_replacer;
mod transform_in_generate;
mod transform_optimizer;
mod transform_provide;
mod transform_react;
// mod tree_shaking;
// mod tree_shaking_module;
mod update;
mod used_ident_collector;
mod watch;
