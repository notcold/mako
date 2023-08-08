use std::any::Any;
use std::sync::Arc;

use anyhow::Result;

use crate::build::FileRequest;
use crate::compiler::Context;
use crate::config::Config;
use crate::load::Content;
use crate::module::ModuleAst;
use crate::stats::StatsJsonMap;

pub struct PluginLoadParam {
    pub path: String,
    pub is_entry: bool,
    pub ext_name: String,
}

pub struct PluginParseParam<'a> {
    pub request: &'a FileRequest,
    pub content: &'a Content,
}

pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> &str;
    fn modify_config(&self, _config: &mut Config) -> Result<()> {
        Ok(())
    }
    fn load(&self, _param: &PluginLoadParam, _context: &Arc<Context>) -> Result<Option<Content>> {
        Ok(None)
    }
    fn parse(
        &self,
        _param: &PluginParseParam,
        _context: &Arc<Context>,
    ) -> Result<Option<ModuleAst>> {
        Ok(None)
    }
    fn build_success(&self, _stats: &StatsJsonMap, _context: &Arc<Context>) -> Result<Option<()>> {
        Ok(None)
    }

    fn runtime_plugins(&self, _context: &Arc<Context>) -> Result<Vec<String>> {
        Ok(Vec::new())
    }
}

#[derive(Default)]
pub struct PluginDriver {
    plugins: Vec<Arc<dyn Plugin>>,
}
impl PluginDriver {
    pub fn new(plugins: Vec<Arc<dyn Plugin>>) -> Self {
        Self { plugins }
    }
    pub fn modify_config(&self, config: &mut Config) -> Result<()> {
        for plugin in &self.plugins {
            plugin.modify_config(config)?;
        }
        Ok(())
    }
    pub fn load(&self, param: &PluginLoadParam, context: &Arc<Context>) -> Result<Option<Content>> {
        for plugin in &self.plugins {
            let ret = plugin.load(param, context)?;
            if ret.is_some() {
                return Ok(ret);
            }
        }
        Ok(None)
    }
    pub fn parse(
        &self,
        param: &PluginParseParam,
        context: &Arc<Context>,
    ) -> Result<Option<ModuleAst>> {
        for plugin in &self.plugins {
            let ret = plugin.parse(param, context)?;
            if ret.is_some() {
                return Ok(ret);
            }
        }
        Ok(None)
    }
    pub fn build_success(
        &self,
        stats: &StatsJsonMap,
        context: &Arc<Context>,
    ) -> Result<Option<()>> {
        for plugin in &self.plugins {
            plugin.build_success(stats, context)?;
        }
        Ok(None)
    }

    pub fn runtime_plugins_code(&self, context: &Arc<Context>) -> Result<String> {
        let mut plugins = Vec::new();
        for plugin in &self.plugins {
            plugins.append(&mut plugin.runtime_plugins(context)?);
        }
        Ok(plugins.join("\n"))
    }
}
