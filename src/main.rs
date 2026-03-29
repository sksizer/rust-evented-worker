use cmd_spec::ShellCommand;
use evented_worker::api::events::{Event, EventStream};
use evented_worker::fixtures::{get_registry, get_test_activity_modules};
use evented_worker::runner::{Controller, Registry, resolve_prior_output};
use evented_worker::activities::shell::{ActivityParameters, get_activity};
use evented_worker::{runner, view};
use log::trace;
use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {}
