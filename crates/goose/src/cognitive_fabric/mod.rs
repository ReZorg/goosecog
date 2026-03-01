//! Cognitive Interface Fabric for OpenCog-inspired distributed cognition
//!
//! This module provides a cognitive interface fabric that enables:
//! - Distributed cognition across multiple agents
//! - Self-healing and recovery mechanisms
//! - Autognosis (self-introspection) and Autogenesis (self-evolution)
//! - Autobuild configuration framework

pub mod autobuild;
pub mod autogenesis;
pub mod autognosis;
pub mod coordinator;
pub mod self_healing;
pub mod types;

pub use autobuild::AutobuildFramework;
pub use autogenesis::AutogenesisEngine;
pub use autognosis::AutognosisEngine;
pub use coordinator::CognitiveCoordinator;
pub use self_healing::SelfHealingManager;
pub use types::{CognitiveEvent, CognitiveState, DistributedKnowledge};
