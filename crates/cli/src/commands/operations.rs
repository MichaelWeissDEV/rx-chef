//! Registry discovery: `operations`, `operation`, `list`, and `info`.

use std::io::{self, Write};

use rxchef::runtime;

use crate::cli::{InfoArgs, ListArgs, OperationAction, OperationArgs, OperationsArgs};
use crate::output;

pub(crate) fn cmd_operations(a: OperationsArgs) -> Result<(), String> {
    let mut operations = rxchef::integration::operations()?;
    if !a.all {
        operations.retain(|operation| operation.availability == rxchef::Availability::Available);
    }
    if let Some(search) = a.search {
        let search = search.to_ascii_lowercase();
        operations.retain(|operation| {
            operation.name.to_ascii_lowercase().contains(&search)
                || operation.id.to_ascii_lowercase().contains(&search)
                || operation.description.to_ascii_lowercase().contains(&search)
        });
    }
    if let Some(module) = a.module {
        operations.retain(|operation| operation.module.eq_ignore_ascii_case(&module));
    }
    if let Some(status) = a.status {
        let normalized = status.to_ascii_lowercase().replace('_', "-");
        if normalized == "feature-gated" {
            operations.retain(|operation| {
                operation.availability == rxchef::Availability::FeatureDisabled
            });
        } else {
            let expected = match normalized.as_str() {
                "complete" => rxchef::ImplementationStatus::Complete,
                "partial" => rxchef::ImplementationStatus::Partial,
                "unsupported" => rxchef::ImplementationStatus::Unsupported,
                "experimental" => rxchef::ImplementationStatus::Experimental,
                _ => {
                    return Err(format!(
                    "unknown operation status '{status}'; expected complete, partial, unsupported, feature-gated, or experimental"
                ));
                }
            };
            operations.retain(|operation| operation.implementation_status == expected);
        }
    }
    let stdout = io::stdout();
    let mut out = stdout.lock();
    if a.json {
        let mut encoded =
            serde_json::to_vec_pretty(&operations).map_err(|error| error.to_string())?;
        encoded.push(b'\n');
        output::write_bytes(&mut out, &encoded)?;
    } else {
        for operation in &operations {
            if let Err(error) = writeln!(
                out,
                "{:<28} {:<18} {:<14?} {}",
                operation.name,
                operation.module,
                operation.implementation_status,
                operation.description
            ) {
                if error.kind() == io::ErrorKind::BrokenPipe {
                    return Ok(());
                }
                return Err(error.to_string());
            }
        }
        eprintln!("\n{} operation(s)", operations.len());
    }
    Ok(())
}

pub(crate) fn cmd_operation(a: OperationArgs) -> Result<(), String> {
    match a.action {
        OperationAction::Describe { operation, json } => {
            let descriptor = rxchef::integration::describe(&operation)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&descriptor).map_err(|error| error.to_string())?
                );
            } else {
                print_operation_descriptor(&descriptor);
            }
        }
    }
    Ok(())
}

// ─── List ─────────────────────────────────────────────────────────────────────

pub(crate) fn cmd_list(a: ListArgs) -> Result<(), String> {
    let names = runtime::operation_names(a.search.as_deref());
    let stdout = io::stdout();
    let mut out = stdout.lock();
    if a.json {
        let v: Vec<_> = names.iter().map(|n| serde_json::json!(n)).collect();
        let mut encoded = serde_json::to_vec_pretty(&v).map_err(|error| error.to_string())?;
        encoded.push(b'\n');
        output::write_bytes(&mut out, &encoded)?;
        return Ok(());
    }
    if a.modules {
        for (module, name) in runtime::operation_names_with_modules(a.search.as_deref())? {
            if let Err(error) = writeln!(out, "{:<32} {}", module, name) {
                if error.kind() == io::ErrorKind::BrokenPipe {
                    return Ok(());
                }
                return Err(error.to_string());
            }
        }
    } else {
        for n in &names {
            if let Err(error) = writeln!(out, "{}", n) {
                if error.kind() == io::ErrorKind::BrokenPipe {
                    return Ok(());
                }
                return Err(error.to_string());
            }
        }
    }
    eprintln!("\n{} operation(s)", names.len());
    Ok(())
}

// ─── Info ─────────────────────────────────────────────────────────────────────

pub(crate) fn cmd_info(a: InfoArgs) -> Result<(), String> {
    let descriptor = rxchef::integration::describe(&a.operation)?;
    if a.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&descriptor).map_err(|error| error.to_string())?
        );
        return Ok(());
    }
    print_operation_descriptor(&descriptor);
    Ok(())
}

pub(crate) fn print_operation_descriptor(descriptor: &rxchef::integration::OperationDescriptor) {
    println!("Name:              {}", descriptor.name);
    println!("ID:                {}", descriptor.id);
    println!("Category:          {}", descriptor.module);
    println!("Description:       {}", descriptor.description);
    println!("Input type:        {}", descriptor.input_type);
    println!("Input requirement: {:?}", descriptor.input_requirement);
    println!("Output type:       {}", descriptor.output_type);
    println!("Implementation:    {:?}", descriptor.implementation_status);
    println!("Availability:      {:?}", descriptor.availability);
    println!(
        "Features:          {}",
        if descriptor.feature_requirements.is_empty() {
            "none".to_string()
        } else {
            descriptor.feature_requirements.join(", ")
        }
    );
    println!("Side effects:      {:?}", descriptor.side_effects);
    println!("Deterministic:     {}", descriptor.deterministic);
    println!("CyberChef parity:  {:?}", descriptor.parity);
    if descriptor.args.is_empty() {
        println!("Arguments:         none");
    } else {
        println!("Arguments:");
        for argument in &descriptor.args {
            println!(
                "  {} ({:?}, required={}, default={:?}, sensitive={})\n      {}",
                argument.name,
                argument.kind,
                argument.required,
                argument.default,
                argument.sensitive,
                argument.description
            );
        }
    }
}

// ─── Run ──────────────────────────────────────────────────────────────────────
