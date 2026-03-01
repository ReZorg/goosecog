//! Cognitive Interface Fabric for OpenCog-inspired distributed cognition
//!
//! This module provides a cognitive interface fabric that enables:
//! - Distributed cognition across multiple agents
//! - Self-healing and recovery mechanisms
//! - Autognosis (self-introspection) and Autogenesis (self-evolution)
//! - Autobuild configuration framework

pub mod types;
pub mod coordinator;
pub mod self_healing;
pub mod autognosis;
pub mod autogenesis;
pub mod autobuild;

pub use types::{CognitiveState, CognitiveEvent, DistributedKnowledge};
pub use coordinator::CognitiveCoordinator;
pub use self_healing::SelfHealingManager;
pub use autognosis::AutognosisEngine;
pub use autogenesis::AutogenesisEngine;
pub use autobuild::AutobuildFramework;
