//! Tool watcher for monitoring tool directories and hot-reloading.
//! 
//! This module provides functionality for watching tool directories
//! and automatically reloading tools when they change.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use notify::{Watcher, RecursiveMode, WatcherKind};
use serde::{Deserialize, Serialize};

use crate::types::IndubitablyResult;
use super::registry::{Tool, ToolRegistry};

/// Configuration for the tool watcher.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolWatcherConfig {
    /// The directory to watch for tools.
    pub watch_directory: PathBuf,
    /// Whether to watch subdirectories recursively.
    pub recursive: bool,
    /// File extensions to watch for.
    pub file_extensions: Vec<String>,
    /// Debounce time for file changes in milliseconds.
    pub debounce_ms: u64,
    /// Whether to enable hot reloading.
    pub enable_hot_reload: bool,
}

impl Default for ToolWatcherConfig {
    fn default() -> Self {
        Self {
            watch_directory: PathBuf::from("./tools"),
            recursive: true,
            file_extensions: vec!["rs".to_string(), "toml".to_string()],
            debounce_ms: 1000,
            enable_hot_reload: true,
        }
    }
}

impl ToolWatcherConfig {
    /// Create a new tool watcher configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the directory to watch.
    pub fn with_watch_directory(mut self, directory: PathBuf) -> Self {
        self.watch_directory = directory;
        self
    }

    /// Set whether to watch recursively.
    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    /// Set the file extensions to watch.
    pub fn with_file_extensions(mut self, extensions: Vec<String>) -> Self {
        self.file_extensions = extensions;
        self
    }

    /// Set the debounce time.
    pub fn with_debounce_ms(mut self, debounce_ms: u64) -> Self {
        self.debounce_ms = debounce_ms;
        self
    }

    /// Enable or disable hot reloading.
    pub fn with_hot_reload(mut self, enable: bool) -> Self {
        self.enable_hot_reload = enable;
        self
    }
}

/// Events that can occur during tool watching.
#[derive(Debug, Clone)]
pub enum ToolWatcherEvent {
    /// A tool file was created.
    ToolCreated(PathBuf),
    /// A tool file was modified.
    ToolModified(PathBuf),
    /// A tool file was deleted.
    ToolDeleted(PathBuf),
    /// A tool was loaded.
    ToolLoaded(String),
    /// A tool was unloaded.
    ToolUnloaded(String),
    /// An error occurred during watching.
    Error(String),
}

/// A watcher for monitoring tool directories and hot-reloading tools.
#[derive(Debug)]
pub struct ToolWatcher {
    config: ToolWatcherConfig,
    registry: Arc<ToolRegistry>,
    watcher: Option<notify::RecommendedWatcher>,
    event_sender: mpsc::Sender<ToolWatcherEvent>,
    event_receiver: mpsc::Receiver<ToolWatcherEvent>,
    loaded_tools: Arc<RwLock<HashMap<PathBuf, String>>>,
}

impl ToolWatcher {
    /// Create a new tool watcher.
    pub fn new(config: ToolWatcherConfig, registry: Arc<ToolRegistry>) -> IndubitablyResult<Self> {
        let (event_sender, event_receiver) = mpsc::channel(100);
        let loaded_tools = Arc::new(RwLock::new(HashMap::new()));

        Ok(Self {
            config,
            registry,
            watcher: None,
            event_sender,
            event_receiver,
            loaded_tools,
        })
    }

    /// Start watching the tool directory.
    pub async fn start(&mut self) -> IndubitablyResult<()> {
        if !self.config.enable_hot_reload {
            return Ok(());
        }

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            tx.send(res).unwrap();
        })?;

        let watch_path = &self.config.watch_directory;
        if !watch_path.exists() {
            std::fs::create_dir_all(watch_path)?;
        }

        let recursive_mode = if self.config.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        watcher.watch(watch_path, recursive_mode)?;
        self.watcher = Some(watcher);

        // Spawn the event processing task
        let event_sender = self.event_sender.clone();
        let registry = Arc::clone(&self.registry);
        let loaded_tools = Arc::clone(&self.loaded_tools);
        let config = self.config.clone();

        tokio::spawn(async move {
            Self::process_events(rx, event_sender, registry, loaded_tools, config).await;
        });

        // Load existing tools
        self.load_existing_tools().await?;

        Ok(())
    }

    /// Stop watching the tool directory.
    pub fn stop(&mut self) {
        self.watcher = None;
    }

    /// Get the next event from the watcher.
    pub async fn next_event(&mut self) -> Option<ToolWatcherEvent> {
        self.event_receiver.recv().await
    }

    /// Check if the watcher is running.
    pub fn is_running(&self) -> bool {
        self.watcher.is_some()
    }

    /// Load existing tools from the watch directory.
    async fn load_existing_tools(&self) -> IndubitablyResult<()> {
        if !self.config.watch_directory.exists() {
            return Ok(());
        }

        let entries = std::fs::read_dir(&self.config.watch_directory)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if self.should_watch_file(&path) {
                if let Err(e) = self.load_tool_file(&path).await {
                    tracing::warn!("Failed to load existing tool from {:?}: {}", path, e);
                }
            }
        }

        Ok(())
    }

    /// Check if a file should be watched.
    fn should_watch_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return self.config.file_extensions.contains(&ext_str.to_string());
            }
        }
        false
    }

    /// Load a tool from a file.
    async fn load_tool_file(&self, path: &Path) -> IndubitablyResult<()> {
        // For now, this is a placeholder implementation
        // In a real implementation, you would:
        // 1. Parse the file (Rust source, TOML config, etc.)
        // 2. Extract tool definitions
        // 3. Register them with the registry
        
        let tool_name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Create a placeholder tool
        let tool = Tool::new(
            &tool_name,
            &format!("Tool loaded from {:?}", path),
            Arc::new(|_| Ok(serde_json::Value::String("placeholder".to_string()))),
        );

        self.registry.register(tool).await?;
        
        // Record the loaded tool
        let mut loaded_tools = self.loaded_tools.write().await;
        loaded_tools.insert(path.to_path_buf(), tool_name);

        Ok(())
    }

    /// Unload a tool from a file.
    async fn unload_tool_file(&self, path: &Path) -> IndubitablyResult<()> {
        let mut loaded_tools = self.loaded_tools.write().await;
        
        if let Some(tool_name) = loaded_tools.remove(path) {
            self.registry.unregister(&tool_name).await?;
        }

        Ok(())
    }

    /// Process file system events.
    async fn process_events(
        rx: std::sync::mpsc::Receiver<notify::Result<notify::Event>>,
        event_sender: mpsc::Sender<ToolWatcherEvent>,
        registry: Arc<ToolRegistry>,
        loaded_tools: Arc<RwLock<HashMap<PathBuf, String>>>,
        config: ToolWatcherConfig,
    ) {
        for res in rx {
            match res {
                Ok(event) => {
                    for kind in event.kinds {
                        match kind {
                            notify::EventKind::Create(_) => {
                                for path in &event.paths {
                                    if Self::should_watch_file_static(&config, path) {
                                        if let Err(e) = Self::load_tool_file_static(&registry, &loaded_tools, path).await {
                                            let _ = event_sender.send(ToolWatcherEvent::Error(e.to_string())).await;
                                        } else {
                                            let _ = event_sender.send(ToolWatcherEvent::ToolCreated(path.clone())).await;
                                        }
                                    }
                                }
                            }
                            notify::EventKind::Modify(_) => {
                                for path in &event.paths {
                                    if Self::should_watch_file_static(&config, path) {
                                        if let Err(e) = Self::reload_tool_file_static(&registry, &loaded_tools, path).await {
                                            let _ = event_sender.send(ToolWatcherEvent::Error(e.to_string())).await;
                                        } else {
                                            let _ = event_sender.send(ToolWatcherEvent::ToolModified(path.clone())).await;
                                        }
                                    }
                                }
                            }
                            notify::EventKind::Remove(_) => {
                                for path in &event.paths {
                                    if let Err(e) = Self::unload_tool_file_static(&registry, &loaded_tools, path).await {
                                        let _ = event_sender.send(ToolWatcherEvent::Error(e.to_string())).await;
                                    } else {
                                        let _ = event_sender.send(ToolWatcherEvent::ToolDeleted(path.clone())).await;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Err(e) => {
                    let _ = event_sender.send(ToolWatcherEvent::Error(e.to_string())).await;
                }
            }
        }
    }

    /// Static version of should_watch_file for use in async context.
    fn should_watch_file_static(config: &ToolWatcherConfig, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return config.file_extensions.contains(&ext_str.to_string());
            }
        }
        false
    }

    /// Static version of load_tool_file for use in async context.
    async fn load_tool_file_static(
        registry: &ToolRegistry,
        loaded_tools: &Arc<RwLock<HashMap<PathBuf, String>>>,
        path: &Path,
    ) -> IndubitablyResult<()> {
        let tool_name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let tool = Tool::new(
            &tool_name,
            &format!("Tool loaded from {:?}", path),
            Arc::new(|_| Ok(serde_json::Value::String("placeholder".to_string()))),
        );

        registry.register(tool).await?;
        
        let mut loaded_tools = loaded_tools.write().await;
        loaded_tools.insert(path.to_path_buf(), tool_name);

        Ok(())
    }

    /// Static version of reload_tool_file for use in async context.
    async fn reload_tool_file_static(
        registry: &ToolRegistry,
        loaded_tools: &Arc<RwLock<HashMap<PathBuf, String>>>,
        path: &Path,
    ) -> IndubitablyResult<()> {
        // First unload the existing tool
        Self::unload_tool_file_static(registry, loaded_tools, path).await?;
        
        // Then load the new version
        Self::load_tool_file_static(registry, loaded_tools, path).await
    }

    /// Static version of unload_tool_file for use in async context.
    async fn unload_tool_file_static(
        registry: &ToolRegistry,
        loaded_tools: &Arc<RwLock<HashMap<PathBuf, String>>>,
        path: &Path,
    ) -> IndubitablyResult<()> {
        let mut loaded_tools = loaded_tools.write().await;
        
        if let Some(tool_name) = loaded_tools.remove(path) {
            registry.unregister(&tool_name).await?;
        }

        Ok(())
    }
}

impl Drop for ToolWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_tool_watcher_config() {
        let config = ToolWatcherConfig::new()
            .with_watch_directory(PathBuf::from("./custom_tools"))
            .with_recursive(false)
            .with_file_extensions(vec!["py".to_string()])
            .with_debounce_ms(500)
            .with_hot_reload(false);

        assert_eq!(config.watch_directory, PathBuf::from("./custom_tools"));
        assert!(!config.recursive);
        assert_eq!(config.file_extensions, vec!["py"]);
        assert_eq!(config.debounce_ms, 500);
        assert!(!config.enable_hot_reload);
    }

    #[tokio::test]
    async fn test_tool_watcher_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = ToolWatcherConfig::new()
            .with_watch_directory(temp_dir.path().to_path_buf());
        
        let registry = Arc::new(ToolRegistry::new());
        let watcher = ToolWatcher::new(config, registry);
        
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_should_watch_file() {
        let config = ToolWatcherConfig::new()
            .with_file_extensions(vec!["rs".to_string(), "toml".to_string()]);

        let rust_file = PathBuf::from("test.rs");
        let toml_file = PathBuf::from("config.toml");
        let other_file = PathBuf::from("test.txt");

        assert!(ToolWatcher::should_watch_file_static(&config, &rust_file));
        assert!(ToolWatcher::should_watch_file_static(&config, &toml_file));
        assert!(!ToolWatcher::should_watch_file_static(&config, &other_file));
    }
}
