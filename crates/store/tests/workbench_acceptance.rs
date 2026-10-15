//! Public-contract and filesystem acceptance tests for portable workbench data.
use rxchef::graph::{Graph, GraphPolicy};
use rxchef_store::{
    workbench::{
        InputFileReference, WorkbenchProject, WorkbenchRecipe, WorkbenchRun, WorkbenchRunStatus,
        WorkbenchSettings, WorkbenchStep, WorkbenchTheme, MAX_WORKBENCH_BYTES,
    },
    GraphRepository, GraphStoreLimits, ProjectRepository,
};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    process::{Command, Stdio},
    sync::{Arc, Barrier},
};

struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("rxchef-acceptance-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn session() -> WorkbenchProject {
    let mut project = WorkbenchProject::new("Portable session");
    project.graphs = (0..3)
        .map(|i| Graph::from_recipe(format!("graph-{i}"), vec![]))
        .collect();
    project.recipes = (0..2)
        .map(|i| WorkbenchRecipe {
            name: format!("recipe-{i}"),
            description: "A definition, not execution data".into(),
            steps: vec![WorkbenchStep {
                op: "To Base64".into(),
                args: vec![],
                enabled: i == 0,
                breakpoint: i == 1,
            }],
        })
        .collect();
    project.settings = WorkbenchSettings {
        theme: WorkbenchTheme::Light,
        selected_graph: Some("graph-2".into()),
        selected_recipe: Some("recipe-1".into()),
    };
    project.input_files.push(InputFileReference {
        graph_id: "graph-0".into(),
        input_name: "input".into(),
        path: "does-not-exist/private-input.bin".into(),
        external: true,
    });
    project.history.push(run("history-0"));
    project
}
fn run(id: &str) -> WorkbenchRun {
    WorkbenchRun {
        id: id.into(),
        run_id: format!("run-{id}"),
        graph_id: "graph-0".into(),
        started_at: "2026-10-15T10:00:00+02:00".into(),
        status: WorkbenchRunStatus::Succeeded,
        elapsed_ms: 1.25,
        output_sizes: BTreeMap::from([("output".into(), 259)]),
        error_code: None,
    }
}
fn equivalent(left: &WorkbenchProject, right: &WorkbenchProject) {
    assert_eq!(
        serde_json::to_value(left).unwrap(),
        serde_json::to_value(right).unwrap()
    );
}

#[test]
fn complete_session_survives_export_repository_reopen_and_file_import() {
    let root = Directory::new();
    let project = session();
    let portable = root.0.join("portable.rxproject.json");
    project.save(&portable).unwrap();
    let imported = WorkbenchProject::load(&portable).unwrap();
    equivalent(&project, &imported);
    {
        let store = ProjectRepository::open(&root.0.join("library"), 8, 1 << 20).unwrap();
        store.save("session", &imported, false).unwrap();
    }
    let reopened = ProjectRepository::open(&root.0.join("library"), 8, 1 << 20).unwrap();
    let loaded = reopened.load("session").unwrap();
    equivalent(&project, &loaded);
    let metadata = reopened.list().unwrap();
    assert_eq!((metadata[0].graphs, metadata[0].recipes), (3, 2));
    assert!(matches!(loaded.settings.theme, WorkbenchTheme::Light));
    assert!(!loaded.recipes[1].steps[0].enabled);
    assert!(loaded.recipes[1].steps[0].breakpoint);
    assert!(!root.0.join("does-not-exist").exists());
}

#[test]
fn missing_external_paths_are_metadata_and_are_never_opened_during_validation() {
    for path in [
        "../missing.bin",
        "/unavailable/file.bin",
        "C:\\unavailable\\file.bin",
    ] {
        let mut project = session();
        project.input_files[0].path = path.into();
        let restored = WorkbenchProject::from_json(&project.to_json().unwrap()).unwrap();
        assert_eq!(restored.input_files[0].path, path);
    }
}

#[test]
fn unknown_fields_at_each_persistence_boundary_are_rejected_without_echoing_values() {
    let baseline = serde_json::to_value(session()).unwrap();
    for pointer in [
        "",
        "/settings",
        "/recipes/0",
        "/recipes/0/steps/0",
        "/input_files/0",
        "/history/0",
        "/graphs/0",
    ] {
        let mut document = baseline.clone();
        document
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert(
                "bearer_token".into(),
                serde_json::json!("PRIVATE-SENTINEL-DO-NOT-ECHO"),
            );
        let error =
            WorkbenchProject::from_json(&serde_json::to_vec(&document).unwrap()).unwrap_err();
        assert!(
            !error.to_string().contains("PRIVATE-SENTINEL-DO-NOT-ECHO"),
            "{pointer}"
        );
    }
}

#[test]
fn unsupported_versions_and_malformed_documents_do_not_replace_saved_projects() {
    let root = Directory::new();
    let store = ProjectRepository::open(&root.0, 8, 1 << 20).unwrap();
    let original = session();
    store.save("session", &original, false).unwrap();
    for version in [0, 2, u32::MAX] {
        let mut invalid = original.clone();
        invalid.version = version;
        assert!(store.save("session", &invalid, true).is_err());
        equivalent(&store.load("session").unwrap(), &original);
    }
    for malformed in [b"".as_slice(), b"null", b"[]", b"{bad", b"{\"version\":1}"] {
        assert!(WorkbenchProject::from_json(malformed).is_err());
        equivalent(&store.load("session").unwrap(), &original);
    }
}

#[test]
fn labels_enforce_utf8_byte_limits_without_slicing_inside_characters() {
    for valid in ["a".repeat(256), "🌍".repeat(64), "Grüße 世界".into()] {
        assert!(WorkbenchProject::new(valid).to_json().is_ok());
    }
    for invalid in [
        String::new(),
        "a".repeat(257),
        "🌍".repeat(65),
        "bad\nname".into(),
        "bad\0name".into(),
    ] {
        assert!(WorkbenchProject::new(invalid).to_json().is_err());
    }
}

#[test]
fn project_definition_and_history_count_boundaries_are_enforced() {
    let mut project = WorkbenchProject::new("limits");
    project.graphs = (0..100)
        .map(|i| Graph::from_recipe(format!("g-{i}"), vec![]))
        .collect();
    project.recipes = (0..100)
        .map(|i| WorkbenchRecipe {
            name: format!("r-{i}"),
            description: String::new(),
            steps: vec![],
        })
        .collect();
    project.history = (0..1000).map(|i| run(&format!("history-{i}"))).collect();
    assert!(project.to_json().is_ok());
    let mut extra = project.clone();
    extra
        .graphs
        .push(Graph::from_recipe("overflow".into(), vec![]));
    assert!(extra.validate().is_err());
    let mut extra = project.clone();
    extra.recipes.push(WorkbenchRecipe {
        name: "overflow".into(),
        description: String::new(),
        steps: vec![],
    });
    assert!(extra.validate().is_err());
    project.history.push(run("overflow"));
    assert!(project.validate().is_err());
}

#[test]
fn duplicate_definitions_selections_references_and_history_ids_fail_explicitly() {
    let baseline = session();
    let mut project = baseline.clone();
    project.graphs.push(project.graphs[0].clone());
    assert!(project.validate().is_err());
    let mut project = baseline.clone();
    project.recipes.push(project.recipes[0].clone());
    assert!(project.validate().is_err());
    let mut project = baseline.clone();
    project.settings.selected_graph = Some("missing".into());
    assert!(project.validate().is_err());
    let mut project = baseline.clone();
    project.settings.selected_recipe = Some("missing".into());
    assert!(project.validate().is_err());
    let mut project = baseline.clone();
    project.input_files.push(project.input_files[0].clone());
    assert!(project.validate().is_err());
    let mut project = baseline;
    project.history.push(project.history[0].clone());
    assert!(project.validate().is_err());
}

#[test]
fn invalid_references_reject_embedded_payload_intent_and_unbound_names() {
    for change in [0, 1, 2, 3, 4] {
        let mut project = session();
        let reference = &mut project.input_files[0];
        match change {
            0 => reference.external = false,
            1 => reference.graph_id = "missing".into(),
            2 => reference.input_name = "missing".into(),
            3 => reference.path.clear(),
            _ => reference.path = "bad\0path".into(),
        }
        assert!(project.validate().is_err());
    }
}

#[test]
fn history_rejects_nonfinite_negative_excessive_and_malformed_metadata() {
    for elapsed in [f64::NAN, f64::INFINITY, -1.0, 604800001.0] {
        let mut project = session();
        project.history[0].elapsed_ms = elapsed;
        assert!(project.validate().is_err());
    }
    for timestamp in ["", "yesterday", "2026-99-99T00:00:00Z", "2026-10-15"] {
        let mut project = session();
        project.history[0].started_at = timestamp.into();
        assert!(project.validate().is_err());
    }
    let mut project = session();
    project.history[0]
        .output_sizes
        .insert("bad\u{1b}name".into(), 1);
    assert!(project.validate().is_err());
    let mut project = session();
    project.history[0].graph_id = "removed-graph".into();
    assert!(
        project.validate().is_ok(),
        "historical metadata may outlive a deleted graph"
    );
}

#[test]
fn secret_literals_in_disabled_and_embedded_recipes_are_never_exported() {
    for enabled in [true, false] {
        let mut project = session();
        project.recipes[0].steps[0] = WorkbenchStep {
            op: "AES Decrypt".into(),
            args: vec!["PRIVATE-KEY-SENTINEL".into()],
            enabled,
            breakpoint: false,
        };
        let error = project.to_json().unwrap_err().to_string();
        assert!(!error.contains("PRIVATE-KEY-SENTINEL"));
    }
    let mut project = session();
    project.graphs[0] = Graph::from_recipe(
        "graph-0".into(),
        vec![rxchef::execution::RecipeStep {
            op: "AES Decrypt".into(),
            args: vec!["PRIVATE-KEY-SENTINEL".into()],
        }],
    );
    assert!(project.to_json().is_err());
}

#[test]
fn project_byte_quota_accepts_exact_boundary_and_failed_replacement_is_atomic() {
    let root = Directory::new();
    let project = session();
    let bytes = project.to_json().unwrap();
    let store = ProjectRepository::open(&root.0, 1, bytes.len()).unwrap();
    store.save("one", &project, false).unwrap();
    let mut larger = project.clone();
    larger.recipes[0].description.push_str("larger");
    assert!(store.save("one", &larger, true).is_err());
    assert!(store.save("two", &project, false).is_err());
    equivalent(&store.load("one").unwrap(), &project);
    assert_eq!(
        std::fs::read(root.0.join("one.rxproject.json")).unwrap(),
        bytes
    );
    assert!(std::fs::read_dir(&root.0).unwrap().all(|entry| !entry
        .unwrap()
        .file_name()
        .to_string_lossy()
        .ends_with(".tmp")));
}

#[test]
fn rename_collision_preserves_both_entries_and_duplicate_has_independent_identity() {
    let root = Directory::new();
    let store = ProjectRepository::open(&root.0, 8, 1 << 20).unwrap();
    let original = session();
    store.save("first", &original, false).unwrap();
    store.duplicate("first", "second").unwrap();
    assert_eq!(store.load("second").unwrap().name, "second");
    assert!(store.rename("first", "SECOND").is_err());
    assert!(store.rename("first", "first").is_err());
    equivalent(&store.load("first").unwrap(), &original);
    store.rename("first", "renamed").unwrap();
    equivalent(&store.load("renamed").unwrap(), &original);
    assert!(store.load("first").is_err());
    store.delete("second").unwrap();
    assert_eq!(store.list().unwrap().len(), 1);
}

#[test]
fn portable_storage_names_are_rejected_consistently_in_graph_and_project_repositories() {
    let root = Directory::new();
    let projects = ProjectRepository::open(&root.0.join("projects"), 8, 1 << 20).unwrap();
    let graphs = GraphRepository::open(
        &root.0.join("graphs"),
        GraphStoreLimits::default(),
        GraphPolicy::default(),
    )
    .unwrap();
    for name in [
        "../escape",
        "a/b",
        "a\\b",
        "CON",
        "con",
        "NUL",
        "PRN",
        "COM0",
        "LPT9",
        "a.",
        "a ",
        "é",
        "e\u{301}",
        "x:y",
        "\0",
    ] {
        assert!(projects.save(name, &session(), false).is_err(), "{name:?}");
        assert!(
            graphs
                .save(name, &Graph::from_recipe("g".into(), vec![]), false)
                .is_err(),
            "{name:?}"
        );
    }
    assert!(projects.list().unwrap().is_empty());
    assert!(graphs.list().unwrap().is_empty());
}

#[test]
fn independent_repository_instances_share_the_same_threaded_quota_lock() {
    let root = Directory::new();
    let barrier = Arc::new(Barrier::new(8));
    let threads: Vec<_> = (0..8)
        .map(|i| {
            let path = root.0.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let store = ProjectRepository::open(&path, 3, 1 << 20).unwrap();
                barrier.wait();
                store.save(&format!("p-{i}"), &session(), false).is_ok()
            })
        })
        .collect();
    assert_eq!(
        threads
            .into_iter()
            .map(|thread| thread.join().unwrap())
            .filter(|success| *success)
            .count(),
        3
    );
    assert_eq!(
        ProjectRepository::open(&root.0, 3, 1 << 20)
            .unwrap()
            .list()
            .unwrap()
            .len(),
        3
    );
}

#[test]
fn process_locking_serializes_project_quota() {
    if let Some(root) = std::env::var_os("RXCHEF_ACCEPTANCE_CHILD_ROOT") {
        let store = ProjectRepository::open(&PathBuf::from(root), 3, 1 << 20).unwrap();
        let key = std::env::var("RXCHEF_ACCEPTANCE_CHILD_KEY").unwrap();
        match store.save(&key, &session(), false) {
            Ok(()) => std::process::exit(0),
            Err(rxchef_store::StoreError::Other(message))
                if message == "Project store file limit exceeded" =>
            {
                std::process::exit(42)
            }
            Err(error) => panic!("unexpected child save failure: {error}"),
        }
    }
    let root = Directory::new();
    let executable = std::env::current_exe().unwrap();
    let children: Vec<_> = (0..8)
        .map(|i| {
            Command::new(&executable)
                .args(["--exact", "process_locking_serializes_project_quota"])
                .env("RXCHEF_ACCEPTANCE_CHILD_ROOT", &root.0)
                .env("RXCHEF_ACCEPTANCE_CHILD_KEY", format!("p-{i}"))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap()
        })
        .collect();
    let results: Vec<_> = children
        .into_iter()
        .map(|mut child| child.wait().unwrap().code().unwrap())
        .collect();
    assert!(
        results.iter().all(|code| matches!(code, 0 | 42)),
        "{results:?}"
    );
    assert_eq!(results.iter().filter(|code| **code == 0).count(), 3);
    assert_eq!(
        ProjectRepository::open(&root.0, 3, 1 << 20)
            .unwrap()
            .list()
            .unwrap()
            .len(),
        3
    );
}

#[test]
fn portable_file_byte_limit_is_checked_before_parsing() {
    let root = Directory::new();
    let path = root.0.join("oversized.json");
    std::fs::File::create(&path)
        .unwrap()
        .set_len(MAX_WORKBENCH_BYTES as u64 + 1)
        .unwrap();
    assert!(WorkbenchProject::load(&path)
        .unwrap_err()
        .to_string()
        .contains("byte limit"));
}

#[cfg(unix)]
#[test]
fn graph_and_project_files_and_locks_are_private_and_symlink_roots_are_refused() {
    use std::os::unix::fs::{symlink, PermissionsExt};
    let root = Directory::new();
    let path = root.0.join("projects");
    let store = ProjectRepository::open(&path, 8, 1 << 20).unwrap();
    store.save("session", &session(), false).unwrap();
    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o700
    );
    for name in ["session.rxproject.json", ".projects.lock"] {
        assert_eq!(
            std::fs::metadata(path.join(name))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
    let link = root.0.join("link");
    symlink(&path, &link).unwrap();
    assert!(ProjectRepository::open(&link, 8, 1 << 20).is_err());
    assert!(
        GraphRepository::open(&link, GraphStoreLimits::default(), GraphPolicy::default()).is_err()
    );
}
