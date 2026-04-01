use crate::api::events::{Event, EventStream, SystemEvent};
use crate::api::activities::ActivityEvent;
use colored::Colorize;
use crate::view::summarize::REPEAT;

pub fn event_stream(events: &EventStream) {
    println!("{}", "─".repeat(REPEAT));
    println!("{} {}", "Event Stream:".bold(), format!("({} events)", events.len()).dimmed());
    println!("{}", "─".repeat(REPEAT));

    if events.is_empty() {
        println!("  {}", "(no events)".dimmed());
        println!("{}", ".".repeat(REPEAT));
        return;
    }

    for (i, event) in events.iter().enumerate() {
        match event {
            Event::Activity(activity_event) => {
                let (icon, label, id) = match activity_event {
                    ActivityEvent::AddSync(p) => ("⊕".white(), "AddSync".white(), p.id.as_str()),
                    ActivityEvent::AddAsync(p) => ("⊕".cyan(), "AddAsync".cyan(), p.id.as_str()),
                    ActivityEvent::Start(id) => ("▶".cyan(), "Start".cyan(), id.as_str()),
                    ActivityEvent::Complete(p) => ("✔".green(), "Complete".green(), p.id.as_str()),
                    ActivityEvent::Failed(p) => ("✘".red(), "Failed".red(), p.id.as_str()),
                    ActivityEvent::Error(p) => ("⚠".yellow(), "Error".yellow(), p.id.as_str()),
                };
                println!("  {} {:>2}. {} [{}]", icon, i + 1, label, id.dimmed());

                match activity_event {
                    ActivityEvent::AddSync(p) | ActivityEvent::AddAsync(p) => {
                        println!("         {} {}", "kind:".dimmed(), p.kind);
                        if let Some(config) = &p.config {
                            println!(
                                "         {} {}",
                                "config:".dimmed(),
                                serde_json::to_string(config).unwrap_or_default()
                            );
                        }
                        if let Some(deps) = &p.depends_on {
                            if !deps.is_empty() {
                                println!("         {} {}", "depends_on:".dimmed(), deps.join(", "));
                            }
                        }
                    }
                    ActivityEvent::Complete(p) => {
                        if let Some(output) = &p.output {
                            println!(
                                "         {} {}",
                                "output:".dimmed(),
                                serde_json::to_string_pretty(output)
                                    .unwrap_or_default()
                                    .green()
                            );
                        }
                    }
                    ActivityEvent::Failed(p) | ActivityEvent::Error(p) => {
                        if let Some(reason) = &p.reason {
                            let colored_reason = if matches!(activity_event, ActivityEvent::Error(_)) {
                                reason.as_str().yellow().to_string()
                            } else {
                                reason.as_str().red().to_string()
                            };
                            println!("         {} {}", "reason:".dimmed(), colored_reason);
                        }
                    }
                    ActivityEvent::Start(_) => {}
                }
            }
            Event::System(system_event) => {
                match system_event {
                    SystemEvent::Error(data) => {
                        println!(
                            "  {} {:>2}. {} [{}]",
                            "⚠".yellow(),
                            i + 1,
                            "SystemError".yellow(),
                            data.activity_id.as_str().dimmed()
                        );
                        println!("         {} {}", "source:".dimmed(), data.source);
                        if !data.errors.is_empty() {
                            println!(
                                "         {} {}",
                                "errors:".dimmed(),
                                data.errors.join("; ").yellow()
                            );
                        }
                    }
                    SystemEvent::NoProvider(kind) => {
                        println!(
                            "  {} {:>2}. {} [{}]",
                            "◌".yellow(),
                            i + 1,
                            "NoProvider".yellow(),
                            kind.as_str().dimmed()
                        );
                    }
                }
            }
        }
    }

    println!("{}", ".".repeat(REPEAT));
}
