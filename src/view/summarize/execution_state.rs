use crate::api::activities::{Activity, AsyncActivity, SyncActivity};
use crate::api::execution::{DefaultExecutionState, ExecutionState, ExecutionStatus};
use crate::view::summarize::REPEAT;
use colored::{ColoredString, Colorize};
use petgraph::Direction;
use petgraph::graph::NodeIndex;

fn activity_icon(activity: &Activity) -> (ColoredString, ColoredString) {
    match activity {
        Activity::Sync(SyncActivity::New(_)) => ("◌".white(), "New".white()),
        Activity::Sync(SyncActivity::UnfulfilledDependencies(_)) => {
            ("◇".white(), "Waiting".white())
        }
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
    }
}

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
        println!("{}", ".".repeat(REPEAT));
        return;
    }

    for (i, activity) in execution_state.activities().enumerate() {
        let (icon, status_label) = activity_icon(activity);
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

        let core = activity.core();
        if core.attempt > 0 || core.failure_count > 0 || core.error_count > 0 {
            println!(
                "      {} attempt: {}, failures: {}, errors: {}",
                "retries:".dimmed(),
                core.attempt,
                core.failure_count,
                core.error_count,
            );
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

    show_dependency_graph(execution_state);

    println!("{}", ".".repeat(REPEAT));
}

fn lane_row(lanes: &[Option<NodeIndex>], node_col: usize, marker: &str) -> String {
    let width = lanes.len().max(node_col + 1);
    let mut row = String::new();
    for c in 0..width {
        if c == node_col {
            row.push_str(marker);
        } else if c < lanes.len() && lanes[c].is_some() {
            row.push_str("| ");
        } else {
            row.push_str("  ");
        }
    }
    row
}

fn show_dependency_graph(execution_state: &DefaultExecutionState) {
    let graph = &execution_state.activity_graph;
    if graph.edge_count() == 0 {
        return;
    }

    println!();
    println!("  {}", "Execution Graph:".bold());
    println!("{}", "─".repeat(REPEAT));

    let Ok(topo_order) = petgraph::algo::toposort(graph, None) else {
        println!(
            "  {}",
            "(cycle detected — graph cannot be displayed)".yellow()
        );
        return;
    };

    // lanes[i] = Some(dest) means lane i carries an open edge heading toward `dest`
    let mut lanes: Vec<Option<NodeIndex>> = Vec::new();

    for &node in &topo_order {
        // Lanes that are pointing to this node (its predecessors' open edges)
        let mut incoming: Vec<usize> = lanes
            .iter()
            .enumerate()
            .filter(|(_, l)| **l == Some(node))
            .map(|(i, _)| i)
            .collect();
        incoming.sort_unstable();

        // This node occupies the leftmost incoming lane, or a new lane if it's a root
        let my_col = if incoming.is_empty() {
            // Root node: claim first free lane to the right of the current rightmost active lane,
            // or just find the first None slot (reuse gaps), or append.
            lanes.iter().position(|l| l.is_none()).unwrap_or_else(|| {
                lanes.push(None);
                lanes.len() - 1
            })
        } else {
            incoming[0]
        };

        // Render merge connectors: extra incoming lanes fold into my_col one at a time,
        // moving rightward-to-leftward (each `/` shifts one column closer per row).
        if incoming.len() > 1 {
            // Extra lanes are to the right of my_col (guaranteed by sort + leftmost assignment).
            // Walk each one in leftward, column by column.
            let extra: Vec<usize> = incoming[1..].to_vec();
            for &ec in &extra {
                // Render one `/` row per extra lane, pulling it adjacent to my_col.
                // For simplicity we do a single-step merge row per extra lane.
                let width = lanes.len();
                let row: String = (0..width)
                    .map(|c| {
                        if c == ec {
                            "/ "
                        } else if c == my_col && lanes[c].is_some() {
                            "| "
                        } else if c > my_col && c < ec && lanes[c].is_some() {
                            "| "
                        } else if c < my_col && lanes[c].is_some() {
                            "| "
                        } else {
                            "  "
                        }
                    })
                    .collect();
                println!("  {}", row.trim_end());
                lanes[ec] = None;
            }
        }

        // Render node row: `* ` at my_col, `| ` at other active lanes
        let id = &graph[node];
        let icon: ColoredString = execution_state
            .activity_to_graph_map
            .get(id)
            .map(|a| activity_icon(a).0)
            .unwrap_or_else(|| "?".normal());

        let prefix = lane_row(&lanes, my_col, "* ");
        println!("  {}{} {}", prefix.trim_end(), icon, id);

        // Compute successors
        let successors: Vec<NodeIndex> = graph
            .neighbors_directed(node, Direction::Outgoing)
            .collect();

        // Update lanes for this node's successors
        match successors.as_slice() {
            [] => {
                lanes[my_col] = None;
            }
            [only] => {
                lanes[my_col] = Some(*only);
            }
            [first, rest @ ..] => {
                lanes[my_col] = Some(*first);

                // Assign extra successors to new lanes (preferring slots to the right of my_col)
                let mut branch_cols: Vec<usize> = Vec::new();
                for &succ in rest {
                    let nc = lanes
                        .iter()
                        .enumerate()
                        .skip(my_col + 1)
                        .find(|(_, l)| l.is_none())
                        .map(|(i, _)| i)
                        .unwrap_or_else(|| {
                            lanes.push(None);
                            lanes.len() - 1
                        });
                    lanes[nc] = Some(succ);
                    branch_cols.push(nc);
                }

                // Render branch connector rows: one `\ ` per extra lane, stepping rightward
                let max_bc = branch_cols.iter().copied().max().unwrap_or(my_col);
                let width = lanes.len();
                for bc in (my_col + 1)..=max_bc {
                    let row: String = (0..width)
                        .map(|c| {
                            if c == bc {
                                "\\ "
                            } else if c == my_col && lanes[c].is_some() {
                                "| "
                            } else if c > my_col && c < bc && lanes[c].is_some() {
                                "| "
                            } else if c < my_col && lanes[c].is_some() {
                                "| "
                            } else {
                                "  "
                            }
                        })
                        .collect();
                    println!("  {}", row.trim_end());
                }
            }
        }

        // Trim trailing empty lanes
        while lanes.last() == Some(&None) {
            lanes.pop();
        }
    }
}
