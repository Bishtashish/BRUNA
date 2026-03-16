// bruna_os/src/kernel/registry.rs

use crate::hal::common::{HardwareId, Storage};
use crate::kernel::{Importance, Protocol};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TrustedComponent {
    pub hardware_id: HardwareId,
    pub protocol: Protocol,
    pub importance: Importance,
}

pub struct Registry {
    components: HashMap<HardwareId, TrustedComponent>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: TrustedComponent) {
        self.components.insert(component.hardware_id.clone(), component);
    }

    pub fn get_component(&self, id: &HardwareId) -> Option<&TrustedComponent> {
        self.components.get(id)
    }

    pub fn list_components(&self) -> Vec<&TrustedComponent> {
        self.components.values().collect()
    }

    /// Load registry from storage
    /// For now, this is a placeholder. In a real OS, we would deserialize from bytes.
    pub fn load_from_storage(&mut self, _storage: &dyn Storage) -> Result<(), String> {
        // Mock loading for now as we don't have a serialization format defined yet
        Ok(())
    }

    /// Save registry to storage
    pub fn save_to_storage(&self, _storage: &mut dyn Storage) -> Result<(), String> {
        // Mock saving for now
        Ok(())
    }
}
