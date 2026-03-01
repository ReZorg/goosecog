<div align="center">

# goosecog

_a cognitive interface fabric for opencog distributed cognition, self-healing, and autonomous evolution_

<p align="center">
  <a href="https://opensource.org/licenses/Apache-2.0"
    ><img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg"></a>
  <a href="https://discord.gg/goose-oss"
    ><img src="https://img.shields.io/discord/1287729918100246654?logo=discord&logoColor=white&label=Join+Us&color=blueviolet" alt="Discord"></a>
  <a href="https://github.com/block/goose/actions/workflows/ci.yml"
     ><img src="https://img.shields.io/github/actions/workflow/status/block/goose/ci.yml?branch=main" alt="CI"></a>
</p>
</div>

**goosecog** extends the powerful goose AI agent framework with a cognitive interface fabric inspired by OpenCog, enabling distributed cognition, self-healing mechanisms, and autonomous evolution capabilities.

## Core Features

🧠 **Distributed Cognition** - Coordinate knowledge sharing across multiple agents  
🔧 **Self-Healing** - Automatic detection and recovery from degraded states  
🔍 **Autognosis** - Self-introspection and cognitive state analysis  
🌱 **Autogenesis** - Self-evolution through adaptive proposals  
⚙️ **Autobuild** - Automatic configuration generation

## Quick Start

```rust
use goose::cognitive_fabric::*;

// Initialize cognitive coordinator
let coordinator = CognitiveCoordinator::new();

// Create and register an agent
let mut agent = CognitiveState::default();
agent.capabilities.push("processing".to_string());
coordinator.register_agent(agent).await?;

// Share knowledge across agents
coordinator.share_knowledge(
    agent_id,
    "key".to_string(),
    "value".to_string()
).await?;
```

## Documentation

- [Cognitive Fabric Guide](./COGNITIVE_FABRIC.md) - Complete cognitive fabric documentation
- [Examples](./crates/goose/examples/cognitive_fabric_demo.rs) - Working examples
- [Tests](./crates/goose/tests/cognitive_fabric_test.rs) - Test suite

Run the demo:
```bash
cargo run --example cognitive_fabric_demo -p goose
```

---

## Built on goose

goosecog builds upon the foundation of **goose**, the local, extensible, open source AI agent that automates engineering tasks.

goose is your on-machine AI agent, capable of automating complex development tasks from start to finish. More than just code suggestions, goose can build entire projects from scratch, write and execute code, debug failures, orchestrate workflows, and interact with external APIs - _autonomously_.

### goose Quick Links
- [Quickstart](https://block.github.io/goose/docs/quickstart)
- [Installation](https://block.github.io/goose/docs/getting-started/installation)
- [Tutorials](https://block.github.io/goose/docs/category/tutorials)
- [Documentation](https://block.github.io/goose/docs/category/getting-started)
- [Responsible AI-Assisted Coding Guide](https://github.com/block/goose/blob/main/HOWTOAI.md)
- [Governance](https://github.com/block/goose/blob/main/GOVERNANCE.md)
- [Custom Distributions](https://github.com/block/goose/blob/main/CUSTOM_DISTROS.md) - build your own goose distro with preconfigured providers, extensions, and branding

## Need Help?
- [Diagnostics & Reporting](https://block.github.io/goose/docs/troubleshooting/diagnostics-and-reporting)
- [Known Issues](https://block.github.io/goose/docs/troubleshooting/known-issues)

# a little goose humor 🪿

> Why did the developer choose goose as their AI agent?
> 
> Because it always helps them "migrate" their code to production! 🚀

# goose around with us  
- [Discord](https://discord.gg/goose-oss)
- [YouTube](https://www.youtube.com/@goose-oss)
- [LinkedIn](https://www.linkedin.com/company/goose-oss)
- [Twitter/X](https://x.com/goose_oss)
- [Bluesky](https://bsky.app/profile/opensource.block.xyz)
- [Nostr](https://njump.me/opensource@block.xyz)
