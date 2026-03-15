# Bruna OS

A modular Rust-based operating system designed for the future of distributed intelligence, from UAV swarms to universal IoT integration.

## 🚀 Vision: The Hive Mind
Bruna OS is built on the philosophy that compute, sensors, and displays do not need to be physically tethered. It is the first step towards a seamless integration of all devices—whether it's a refrigerator, a microwave, a home security system, a UAV, or a car.

Our goal is to create a **Hive Mind** architecture that:
- **Abstracts Location:** Logic and hardware are decoupled, allowing a sensor in one room to drive a display in another seamlessly.
- **Universal Integration:** A single unified kernel interface for any device.
- **User-Centric:** The swarm serves the user, with the human at the center of the intelligence network.

## 🏗️ Current State
This branch represents the **Foundational Architecture** phase. We have established the core traits and modular structure that will support the OS's evolution.

- **Microkernel Foundation:** Initial traits for Process Management, Threading, and Scheduling are defined.
- **Modular HAL:** A robust Hardware Abstraction Layer (HAL) framework is in place, with initial support for platforms like the Ryze Tello.
- **Trait-Based Design:** High modularity ensures that any component (scheduler, memory manager, or driver) can be swapped without affecting the rest of the system.

## ✨ Features & Roadmap
Features are listed in increasing order of complexity, representing our journey from a single kernel to a global swarm.

### Phase 1: Foundational (Implemented)
- [x] **Modular Microkernel Architecture:** Core system components defined as interchangeable traits.
- [x] **Basic Multitasking:** Process and Thread management structures with unique ID generation.
- [x] **Round-Robin Scheduler:** A working thread scheduler integrated into the process manager.
- [x] **Hardware Abstraction Layer (HAL):** Unified interfaces for Serial, GPIO, Timers, Network, and Radio.

### Phase 2: Integration (In Progress / Planned)
- [ ] **Advanced IPC:** Robust message-passing between processes and eventually between devices.
- [ ] **Dynamic Memory Management:** Efficient allocation and deallocation for resource-constrained devices.
- [ ] **Physical Driver Implementations:** Moving beyond dummy HALs to support real-world hardware (sensors, motors).
- [ ] **Onboarding Service:** Automated discovery and registration of new nodes in a local network.

### Phase 3: Distributed Systems (Planned)
- [ ] **Network-Transparent Services:** Calling a service on a remote UAV as if it were local.
- [ ] **Swarm Navigation:** Coordinated movement and pattern formation for UAV groups.
- [ ] **Distributed State Consensus:** Ensuring all nodes in the swarm have a consistent view of the world.

### Phase 4: The Vision (Planned)
- [ ] **Universal Device Bridge:** Standardized integration for appliances, vehicles, and home systems.
- [ ] **Distributed "Hive Mind" Intelligence:** Aggregating compute power across the swarm to perform complex tasks.
- [ ] **User-Centric Interface:** A high-level abstraction for humans to interact with the entire hive as a single entity.

## 📦 Modules
- `kernel`: Core microkernel components (Scheduling, Processes, Threads, IPC).
- `hal`: Hardware Abstraction Layer for cross-platform compatibility.
- `drivers`: Device-specific implementations.
- `services`: High-level OS services (Onboarding, Navigation, Swarm Management).
- `comms`: Distributed communication and protocol abstractions.
- `utils`: Shared utilities and data structures.
