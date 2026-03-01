//! Example demonstrating the cognitive interface fabric
//!
//! This example shows how to use the cognitive fabric for distributed cognition,
//! self-healing, autognosis, autogenesis, and autobuild capabilities.
//!
//! Run with: `cargo run --example cognitive_fabric_demo`

use goose::cognitive_fabric::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🧠 Cognitive Interface Fabric Demo");
    println!("===================================\n");

    println!("1. Initializing Cognitive Coordinator...");
    let coordinator = CognitiveCoordinator::new();
    let event_tx = coordinator.get_event_sender();

    println!("2. Creating and registering cognitive agents...\n");
    let mut agent1 = CognitiveState::default();
    agent1.capabilities.push("data_processing".to_string());
    agent1.capabilities.push("analysis".to_string());
    println!("   Agent 1 ID: {}", agent1.agent_id);
    println!("   Capabilities: {:?}", agent1.capabilities);

    let mut agent2 = CognitiveState::default();
    agent2.capabilities.push("decision_making".to_string());
    agent2.capabilities.push("optimization".to_string());
    println!("   Agent 2 ID: {}", agent2.agent_id);
    println!("   Capabilities: {:?}\n", agent2.capabilities);

    coordinator.register_agent(agent1.clone()).await?;
    coordinator.register_agent(agent2.clone()).await?;

    println!("3. Demonstrating Distributed Knowledge Sharing...");
    coordinator
        .share_knowledge(
            agent1.agent_id,
            "algorithm".to_string(),
            "neural_network".to_string(),
        )
        .await?;

    coordinator
        .share_knowledge(
            agent2.agent_id,
            "strategy".to_string(),
            "reinforcement_learning".to_string(),
        )
        .await?;

    println!("   Knowledge shared across distributed agents");

    if let Some(value) = coordinator.get_knowledge("algorithm").await {
        println!("   Retrieved: algorithm = {}", value);
    }

    if let Some(value) = coordinator.get_knowledge("strategy").await {
        println!("   Retrieved: strategy = {}\n", value);
    }

    sleep(Duration::from_millis(100)).await;

    println!("4. Running Autognosis (Self-Introspection)...");
    let autognosis = AutognosisEngine::new();

    agent1.active_tasks.push("task_1".to_string());
    agent1.active_tasks.push("task_2".to_string());

    let report = autognosis.introspect(&agent1).await?;
    println!("   Introspection Report for Agent 1:");
    println!(
        "   - Health Score: {}",
        report.metrics.get("health_score").unwrap_or(&0.0)
    );
    println!(
        "   - Active Tasks: {}",
        report.metrics.get("task_count").unwrap_or(&0.0)
    );
    println!(
        "   - Capabilities: {}",
        report.metrics.get("capability_count").unwrap_or(&0.0)
    );
    println!("   Insights:");
    for insight in &report.insights {
        println!("     • {}", insight);
    }
    println!("   Recommendations:");
    for rec in &report.recommendations {
        println!("     • {}\n", rec);
    }

    println!("5. Testing Self-Healing Mechanisms...");
    let healing_manager = SelfHealingManager::new(event_tx.clone());

    agent1.error_history.push("error_1".to_string());
    agent1.error_history.push("error_2".to_string());
    agent1.error_history.push("error_3".to_string());
    agent1.health_score = SelfHealingManager::calculate_health_score(&agent1);

    println!(
        "   Agent 1 health score after errors: {}",
        agent1.health_score
    );
    healing_manager.check_health(&agent1).await?;
    println!("   Self-healing check completed\n");

    println!("6. Demonstrating Autogenesis (Self-Evolution)...");
    let autogenesis = AutogenesisEngine::new(event_tx.clone());

    let proposals = autogenesis.propose_evolution(&agent1, &report).await?;
    println!(
        "   Evolution proposals generated: {} proposals",
        proposals.len()
    );

    for (i, proposal) in proposals.iter().enumerate() {
        println!("   Proposal {}: {}", i + 1, proposal.description);
        println!(
            "     Expected Impact: +{:.0}%",
            proposal.expected_impact * 100.0
        );
    }

    if !proposals.is_empty() {
        println!("\n   Applying first evolution proposal...");
        autogenesis
            .apply_evolution(&mut agent1, &proposals[0])
            .await?;
        println!(
            "   Evolution applied! New health score: {}\n",
            agent1.health_score
        );
    }

    println!("7. Running Autobuild Configuration Framework...");
    let mut framework = AutobuildFramework::new(event_tx.clone());

    let config = framework.generate_config(&agent1, &report).await?;
    println!(
        "   Auto-configuration generated (version {}):",
        config.version
    );
    println!("   Settings:");
    for (key, value) in &config.settings {
        println!("     • {} = {:?}", key, value);
    }

    framework.apply_config(&mut agent1, &config).await?;
    println!("   Configuration applied to agent\n");

    println!("8. Final Agent State:");
    println!("   Agent ID: {}", agent1.agent_id);
    println!("   Health Score: {}", agent1.health_score);
    println!("   Capabilities: {:?}", agent1.capabilities);
    println!("   Active Tasks: {:?}", agent1.active_tasks);
    println!("   Knowledge Base Entries: {}", agent1.knowledge_base.len());
    println!("   Error History: {} errors\n", agent1.error_history.len());

    coordinator
        .update_agent_state(agent1.agent_id, agent1)
        .await?;

    let all_agents = coordinator.get_agent_states().await;
    println!("9. Summary:");
    println!("   Total agents in system: {}", all_agents.len());
    println!("   Cognitive fabric operational ✓");
    println!("   Distributed cognition active ✓");
    println!("   Self-healing enabled ✓");
    println!("   Autognosis running ✓");
    println!("   Autogenesis operational ✓");
    println!("   Autobuild framework configured ✓\n");

    println!("🎉 Cognitive Interface Fabric Demo Complete!");

    Ok(())
}
