use crate::api::activities::ActivityEvent;
use crate::api::events::{Event, EventStream, SystemEvent};
use crate::view::summarize::REPEAT;
use colored::Colorize;

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
                let (icon, label) = match activity_event {
                    ActivityEvent::AddSync(_) => ("⊕".white(), "Activity::AddSync".white()),
                    ActivityEvent::AddAsync(_) => ("⊕".cyan(), "Activity::AddAsync".cyan()),
                    ActivityEvent::Start(_) => ("▶".cyan(), "Activity::Start".cyan()),
                    ActivityEvent::Complete(_) => ("✔".green(), "Activity::Complete".green()),
                    ActivityEvent::Failed(_) => ("✘".red(), "Activity::Failed".red()),
                    ActivityEvent::Error(_) => ("⚠".yellow(), "Activity::Error".yellow()),
                    ActivityEvent::Retry(_) => ("↻".blue(), "Activity::Retry".blue()),
                };

                println!("  {} {:>2}. {}", icon, i + 1, label);

                let id = activity_event.activity_id();
                println!("         {} {}", "id:".dimmed(), id.bold());

                match activity_event {
                    ActivityEvent::AddSync(p) | ActivityEvent::AddAsync(p) => {
                        println!("         {} {}", "kind:".dimmed(), p.kind.bold());
                        if let Some(config) = &p.config {
                            println!(
                                "         {} {}",
                                "config:".dimmed(),
                                serde_json::to_string(config).unwrap_or_default()
                            );
                        }
                        if let Some(deps) = &p.depends_on
                            && !deps.is_empty()
                        {
                            println!("         {} {}", "depends_on:".dimmed(), deps.join(", "));
                        }
                    }
                    ActivityEvent::Complete(p) => {
                        if let Some(output) = &p.output {
                            println!(
                                "         {} {}",
                                "output:".dimmed(),
                                serde_json::to_string_pretty(output).unwrap_or_default().green()
                            );
                        }
                    }
                    ActivityEvent::Failed(p) => {
                        if let Some(reason) = &p.reason {
                            println!("         {} {}", "reason:".dimmed(), reason.as_str().red());
                        }
                    }
                    ActivityEvent::Error(p) => {
                        if let Some(reason) = &p.reason {
                            println!("         {} {}", "reason:".dimmed(), reason.as_str().yellow());
                        }
                    }
                    ActivityEvent::Start(_) | ActivityEvent::Retry(_) => {}
                }
            }
            Event::System(system_event) => match system_event {
                SystemEvent::Error(data) => {
                    println!("  {} {:>2}. {}", "⚠".yellow(), i + 1, "System::Error".yellow(),);
                    println!("         {} {}", "activity_id:".dimmed(), data.activity_id.bold());
                    println!("         {} {}", "source:".dimmed(), data.source);
                    if !data.errors.is_empty() {
                        println!("         {} {}", "errors:".dimmed(), data.errors.join("; ").yellow());
                    }
                }
                SystemEvent::NoProvider(kind) => {
                    println!("  {} {:>2}. {}", "◌".yellow(), i + 1, "System::NoProvider".yellow(),);
                    println!("         {} {}", "kind:".dimmed(), kind.bold());
                }
            },
        }
    }

    println!("{}", ".".repeat(REPEAT));
}
