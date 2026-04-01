use crate::api::execution::{DefaultExecutionState, ExecutionState, ExecutionStatus};
use crate::api::activities::{AsyncActivity, Activity, SyncActivity};
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

    if execution_state.activity_count() == 0 {
        println!("  {}", "(no activities)".dimmed());
        return;
    }

    for (i, activity) in execution_state.activities().enumerate() {
        let (icon, status_label) = match activity {
            Activity::Sync(SyncActivity::New(_)) => ("◌".white(), "New".white()),
            Activity::Sync(SyncActivity::Ready(_)) => ("○".white(), "Ready".white()),
            Activity::Sync(SyncActivity::Running(_)) => ("●".cyan(), "Running".cyan()),
            Activity::Sync(SyncActivity::Completed(_)) => ("✔".green(), "Completed".green()),
            Activity::Sync(SyncActivity::Failed(_)) => ("✘".red(), "Failed".red()),
            Activity::Sync(SyncActivity::Error(_)) => ("⚠".yellow(), "Error".yellow()),
            Activity::Async(AsyncActivity::Ready(_)) => ("○".white(), "Ready".white()),
            Activity::Async(AsyncActivity::Running(_)) => ("●".cyan(), "Running".cyan()),
            Activity::Async(AsyncActivity::Completed(_)) => ("✔".green(), "Completed".green()),
            Activity::Async(AsyncActivity::Failed(_)) => ("✘".red(), "Failed".red()),
            Activity::Async(AsyncActivity::Error(_)) => ("⚠".yellow(), "Error".yellow()),
        };
        println!(
            "  {} Activity {} [{}] ({}): {}",
            icon,
            i + 1,
            activity.id().dimmed(),
            activity.kind().dimmed(),
            status_label
        );

        if let Some(config) = activity.config() {
            println!("      {} {}", "config:".dimmed(), config);
        }
        if let Some(input) = activity.input() {
            println!("      {} {}", "input:".dimmed(), input);
        }

        match activity {
            Activity::Sync(SyncActivity::Completed(sc)) => {
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
            Activity::Async(AsyncActivity::Completed(ac)) => {
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
            Activity::Sync(SyncActivity::Failed(sf)) => {
                if let Some(reasons) = &sf.failed.failure {
                    println!("      {} {}", "failure:".dimmed(), reasons.join("; ").red());
                }
            }
            Activity::Async(AsyncActivity::Failed(af)) => {
                if let Some(reasons) = &af.failed.failure {
                    println!("      {} {}", "failure:".dimmed(), reasons.join("; ").red());
                }
            }
            Activity::Sync(SyncActivity::Error(se)) => {
                if let Some(reasons) = &se.failed.failure {
                    println!(
                        "      {} {}",
                        "error:".dimmed(),
                        reasons.join("; ").yellow()
                    );
                }
            }
            Activity::Async(AsyncActivity::Error(ae)) => {
                if let Some(reasons) = &ae.failed.failure {
                    println!(
                        "      {} {}",
                        "error:".dimmed(),
                        reasons.join("; ").yellow()
                    );
                }
            }
            _ => {}
        }
    }
    println!("{}", ".".repeat(REPEAT));
}
