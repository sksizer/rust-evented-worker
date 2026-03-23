use colored::Colorize;

use crate::execution_state::{ExecutionState, ExecutionStatus};
use crate::api::steps::{AsyncStep, Step, SyncStep};

pub fn execution_state(execution_state: &ExecutionState) {
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
    println!("{}", "─".repeat(40));

    if execution_state.step_states.is_empty() {
        println!("  {}", "(no steps)".dimmed());
        return;
    }

    for (i, step) in execution_state.step_states.iter().enumerate() {
        let (icon, status_label) = match step {
            Step::Sync(SyncStep::Ready(_)) => ("○".white(), "Ready".white()),
            Step::Sync(SyncStep::Completed { .. }) => ("✔".green(), "Completed".green()),
            Step::Sync(SyncStep::Failed { .. }) => ("✘".red(), "Failed".red()),
            Step::Sync(SyncStep::Error { .. }) => ("⚠".yellow(), "Error".yellow()),
            Step::Async(AsyncStep::Ready(_)) => ("○".white(), "Ready".white()),
            Step::Async(AsyncStep::Running(_)) => ("●".cyan(), "Running".cyan()),
            Step::Async(AsyncStep::Completed { .. }) => ("✔".green(), "Completed".green()),
            Step::Async(AsyncStep::Failed { .. }) => ("✘".red(), "Failed".red()),
            Step::Async(AsyncStep::Error { .. }) => ("⚠".yellow(), "Error".yellow()),
        };
        println!("  {} Step {} [{}] ({}): {}", icon, i + 1, step.id().dimmed(), step.kind().dimmed(), status_label);

        if let Some(input) = &step.core().input {
            println!("      {} {}", "input:".dimmed(), input);
        }

        match step {
            Step::Sync(SyncStep::Completed { output: Some(output), .. })
            | Step::Async(AsyncStep::Completed { output: Some(output), .. }) => {
                println!("      {} {}", "output:".dimmed(), serde_json::to_string_pretty(output).unwrap_or_default().green());
            }
            Step::Sync(SyncStep::Failed { failure: Some(failure), .. })
            | Step::Async(AsyncStep::Failed { failure: Some(failure), .. }) => {
                println!("      {} {}", "failure:".dimmed(), failure.red());
            }
            Step::Sync(SyncStep::Error { failure: Some(failure), .. })
            | Step::Async(AsyncStep::Error { failure: Some(failure), .. }) => {
                println!("      {} {}", "error:".dimmed(), failure.yellow());
            }
            _ => {}
        }
    }
}
