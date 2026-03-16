// bruna_os/src/kernel/discovery.rs

use crate::hal::{PlatformHal, RadioDevice, HardwareId};
use crate::kernel::registry::{Registry, TrustedComponent};
use crate::kernel::{Importance, KernelError, KernelResult};
use std::collections::HashSet;

pub struct DiscoveryManager<'a, H: PlatformHal> {
    hal: &'a H,
    registry: &'a Registry,
}

impl<'a, H: PlatformHal> DiscoveryManager<'a, H> {
    pub fn new(hal: &'a H, registry: &'a Registry) -> Self {
        Self { hal, registry }
    }

    /// Perform a quiet scan of all available hardware interfaces and identify trusted components.
    pub fn scan_and_identify(&self) -> KernelResult<Vec<TrustedComponent>> {
        let mut identified = Vec::new();

        // 1. Scan Radio (if available)
        // Note: For now we assume a single radio device for simplicity
        // In a real implementation we might iterate over multiple.
        let radio = H::Radio::new().map_err(|e| KernelError::Other(format!("{:?}", e)))?;
        if let Ok(visible_ids) = radio.list_visible_devices() {
            for id in visible_ids {
                if let Some(component) = self.registry.get_component(&id) {
                    identified.push(component.clone());
                }
            }
        }

        // 2. Scan Serial (In a real kernel, we might have a list of known serial ports to probe)
        // For demonstration, we check if the platform name matches what we expect from Tello
        if self.hal.platform_name() == "Ryze Tello" {
            // This is a placeholder for actual serial probing
            // In a real system, we'd list /dev/tty* or equivalent
        }

        Ok(identified)
    }

    /// Validate if the currently identified components are sufficient for boot.
    pub fn validate_boot(&self, identified: &[TrustedComponent]) -> KernelResult<()> {
        let identified_ids: HashSet<HardwareId> = identified.iter().map(|c| c.hardware_id.clone()).collect();

        let mut missing_critical = Vec::new();
        let mut _missing_essential = Vec::new();

        for component in self.registry.list_components() {
            if !identified_ids.contains(&component.hardware_id) {
                match component.importance {
                    Importance::Critical => missing_critical.push(component.hardware_id.clone()),
                    Importance::Essential => _missing_essential.push(component.hardware_id.clone()),
                    Importance::Optional => {}
                }
            }
        }

        if !missing_critical.is_empty() {
            return Err(KernelError::InvalidState(format!("Missing critical components: {:?}", missing_critical)));
        }

        // We could log missing_essential here if we had a logger

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::platforms::ryze_tello::TelloHal;
    use crate::hal::common::HardwareId;
    use crate::kernel::registry::TrustedComponent;
    use crate::kernel::{Importance, Protocol};

    #[test]
    fn test_discovery_and_boot() {
        let hal = TelloHal::new();
        let mut registry = Registry::new();

        // Add a critical component that we expect to find
        registry.add_component(TrustedComponent {
            hardware_id: HardwareId("paired_sensor_1".to_string()),
            protocol: Protocol::Radio,
            importance: Importance::Critical,
        });

        let discovery = DiscoveryManager::new(&hal, &registry);
        let identified = discovery.scan_and_identify().unwrap();

        assert_eq!(identified.len(), 1);
        assert_eq!(identified[0].hardware_id, HardwareId("paired_sensor_1".to_string()));

        // Boot should succeed
        assert!(discovery.validate_boot(&identified).is_ok());
    }

    #[test]
    fn test_failed_boot_missing_critical() {
        let hal = TelloHal::new();
        let mut registry = Registry::new();

        // Add a critical component that IS NOT in our dummy HAL's visible list
        registry.add_component(TrustedComponent {
            hardware_id: HardwareId("missing_sensor".to_string()),
            protocol: Protocol::Radio,
            importance: Importance::Critical,
        });

        let discovery = DiscoveryManager::new(&hal, &registry);
        let identified = discovery.scan_and_identify().unwrap();

        // Boot should fail
        let result = discovery.validate_boot(&identified);
        assert!(result.is_err());
        if let Err(KernelError::InvalidState(msg)) = result {
            assert!(msg.contains("Missing critical components"));
        } else {
            panic!("Expected InvalidState error");
        }
    }
}
