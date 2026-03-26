use crate::api::execution::{DefaultExecutionState, ExecutionState, ExecutionStatus};
use crate::api::steps::{AsyncStep, Step, SyncStep};
use colored::Colorize;

static REPEAT: usize = 80;

pub fn execution_state(execution_state: &DefaultExecutionState) {
    println!("{}", "─".repeat(REPEAT));
    let status = execution_state.status();
    let status_text = format!("{:?}", status);
    let colored_status = match status {
        ExecutionStatus::New => status_text.white(),
        ExecutionStatus::Running => status_text.cyan(),
        ExecutionStatus::Finished => status_text.green(),
        ExecutionStatus::Error => status_text.yellow(),
        ExecutionStatus::Failed => status_text.red(),
    };

    println!("{} {}", "Execution Status:".bold(), colored_status.bold());
    println!("{}", "─".repeat(REPEAT));

    if execution_state.step_states.is_empty() {
        println!("  {}", "(no steps)".dimmed());
        return;
    }

    for (i, step) in execution_state.step_states.iter().enumerate() {
        let (icon, status_label) = match step {
            Step::Sync(SyncStep::New(_)) => ("◌".white(), "New".white()),
            Step::Sync(SyncStep::Ready(_)) => ("○".white(), "Ready".white()),
            Step::Sync(SyncStep::Running(_)) => ("●".cyan(), "Running".cyan()),
            Step::Sync(SyncStep::Completed(_)) => ("✔".green(), "Completed".green()),
            Step::Sync(SyncStep::Failed(_)) => ("✘".red(), "Failed".red()),
            Step::Sync(SyncStep::Error(_)) => ("⚠".yellow(), "Error".yellow()),
            Step::Async(AsyncStep::Ready(_)) => ("○".white(), "Ready".white()),
            Step::Async(AsyncStep::Running(_)) => ("●".cyan(), "Running".cyan()),
            Step::Async(AsyncStep::Completed(_)) => ("✔".green(), "Completed".green()),
            Step::Async(AsyncStep::Failed(_)) => ("✘".red(), "Failed".red()),
            Step::Async(AsyncStep::Error(_)) => ("⚠".yellow(), "Error".yellow()),
        };
        println!(
            "  {} Step {} [{}] ({}): {}",
            icon,
            i + 1,
            step.id().dimmed(),
            step.kind().dimmed(),
            status_label
        );

        if let Some(config) = step.config() {
            println!("      {} {}", "config:".dimmed(), config);
        }
        if let Some(input) = step.input() {
            println!("      {} {}", "input:".dimmed(), input);
        }

        match step {
            Step::Sync(SyncStep::Completed(sc)) => {
                if let Some(output) = &sc.completed.output {
                    println!(
                        "      {} {}",
                        "output:".dimmed(),
                        serde_json::to_string_pretty(output)
                            .unwrap_or_default()
                            .green()
                    );
                }
            }
            Step::Async(AsyncStep::Completed(ac)) => {
                if let Some(output) = &ac.completed.output {
                    println!(
                        "      {} {}",
                        "output:".dimmed(),
                        serde_json::to_string_pretty(output)
                            .unwrap_or_default()
                            .green()
                    );
                }
            }
            Step::Sync(SyncStep::Failed(sf)) => {
                if let Some(reasons) = &sf.failed.failure {
                    println!("      {} {}", "failure:".dimmed(), reasons.join("; ").red());
                }
            }
            Step::Async(AsyncStep::Failed(af)) => {
                if let Some(reasons) = &af.failed.failure {
                    println!("      {} {}", "failure:".dimmed(), reasons.join("; ").red());
                }
            }
            Step::Sync(SyncStep::Error(se)) => {
                if let Some(reasons) = &se.failed.failure {
                    println!("      {} {}", "error:".dimmed(), reasons.join("; ").yellow());
                }
            }
            Step::Async(AsyncStep::Error(ae)) => {
                if let Some(reasons) = &ae.failed.failure {
                    println!("      {} {}", "error:".dimmed(), reasons.join("; ").yellow());
                }
            }
            _ => {}
        }
    }
    println!("{}", ".".repeat(REPEAT));
}
