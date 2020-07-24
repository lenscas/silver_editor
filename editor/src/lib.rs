mod editor_context;
mod events;

pub use editor_context::EditorContext;

pub struct CompiledToNativeConfig {
    pub port: u16,
    pub address: [u8; 4],
}
impl Default for CompiledToNativeConfig {
    fn default() -> Self {
        Self {
            port: 3030,
            address: [0, 0, 0, 0],
        }
    }
}
pub enum AttachButtonAt {
    Id(String),
    Element(String),
}
impl Default for AttachButtonAt {
    fn default() -> Self {
        Self::Element("body".into())
    }
}
#[derive(Default)]
pub struct CompiledToWasmConfig {
    pub button_loc: AttachButtonAt,
}

#[derive(Default)]
pub struct EditorConfig {
    pub native_config: CompiledToNativeConfig,
    pub web_config: CompiledToWasmConfig,
}
