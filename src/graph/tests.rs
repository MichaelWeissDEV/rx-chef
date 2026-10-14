use super::*;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::{
    collections::BTreeMap,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

fn node(id: &str, kind: NodeKind) -> Node {
    Node {
        id: id.into(),
        kind,
    }
}
fn input(id: &str) -> Node {
    node(id, NodeKind::Input { name: id.into() })
}
fn output(id: &str) -> Node {
    node(id, NodeKind::Output { name: id.into() })
}
fn op(id: &str, name: &str) -> Node {
    node(
        id,
        NodeKind::Operation {
            operation: crate::runtime::canonical_identifier(name),
            args: BTreeMap::new(),
            enabled: true,
        },
    )
}
fn edge(source: &str, target: &str, port: &str) -> Edge {
    Edge {
        id: format!("{source}-{target}-{port}"),
        source_node: source.into(),
        source_port: "data".into(),
        target_node: target.into(),
        target_port: port.into(),
    }
}
fn graph(nodes: Vec<Node>, edges: Vec<Edge>) -> Graph {
    Graph {
        version: GRAPH_VERSION,
        id: "test".into(),
        nodes,
        edges,
        presentation: BTreeMap::new(),
    }
}
fn run(graph: &Graph, inputs: &[(&str, &[u8])]) -> GraphResult {
    execute(
        graph,
        inputs
            .iter()
            .map(|(k, v)| ((*k).into(), v.to_vec()))
            .collect(),
        &[],
        &GraphPolicy::default(),
        &ExecutionControl::default(),
    )
    .unwrap()
}
fn bytes(result: &GraphResult, name: &str) -> Vec<u8> {
    STANDARD
        .decode(
            result
                .outputs
                .iter()
                .find(|out| out.name == name)
                .unwrap()
                .output_base64
                .as_ref()
                .unwrap(),
        )
        .unwrap()
}
fn binary_graph() -> Graph {
    graph(
        vec![
            input("a"),
            input("b"),
            input("c"),
            op("encode", "To Base64"),
            op("decode", "From Base64"),
            node(
                "concat",
                NodeKind::Combine {
                    combine: Combine::Concatenate { inputs: 3 },
                },
            ),
            output("all"),
            output("encoded"),
            output("original"),
        ],
        vec![
            edge("a", "encode", "data"),
            edge("encode", "decode", "data"),
            edge("decode", "concat", "input:0"),
            edge("b", "concat", "input:1"),
            edge("c", "concat", "input:2"),
            edge("concat", "all", "data"),
            edge("encode", "encoded", "data"),
            edge("decode", "original", "data"),
        ],
    )
}
#[test]
fn three_inputs_fan_in_fan_out_three_named_outputs_preserve_all_bytes() {
    let a: Vec<u8> = (0..=255).collect();
    let b = [0, 255, 192];
    let c = b"text";
    let result = run(&binary_graph(), &[("a", &a), ("b", &b), ("c", c)]);
    assert_eq!(bytes(&result, "original"), a);
    assert_eq!(bytes(&result, "encoded"), STANDARD.encode(&a).as_bytes());
    assert_eq!(bytes(&result, "all"), [a.as_slice(), &b, c].concat());
    assert_eq!(
        result
            .nodes
            .iter()
            .filter(|n| n.node_id == "encode")
            .count(),
        1
    );
    assert!(result
        .nodes
        .iter()
        .all(|n| n.status == NodeStatus::Succeeded));
}
#[test]
fn empty_and_large_binary_inputs_and_roundtrip() {
    let graph = Graph::from_recipe("roundtrip".into(), vec![]);
    let json = serde_json::to_vec(&graph).unwrap();
    let restored = Graph::from_json(&json, &GraphPolicy::default()).unwrap();
    for value in [vec![], vec![0, 255, 0, 192], vec![255; 1 << 20]] {
        assert_eq!(
            bytes(&run(&restored, &[("input", &value)]), "output"),
            value
        );
    }
}
#[test]
fn deterministic_order_ignores_definition_order_and_positions() {
    let mut g = binary_graph();
    let first = validate(&g, &GraphPolicy::default())
        .unwrap()
        .order
        .iter()
        .map(|i| g.nodes[*i].id.clone())
        .collect::<Vec<_>>();
    g.nodes.reverse();
    g.edges.reverse();
    g.presentation.insert(
        "a".into(),
        Position {
            x: 999.0,
            y: -400.0,
        },
    );
    let second = validate(&g, &GraphPolicy::default())
        .unwrap()
        .order
        .iter()
        .map(|i| g.nodes[*i].id.clone())
        .collect::<Vec<_>>();
    assert_eq!(first, second);
}
#[test]
fn failures_are_isolated_and_downstream_is_blocked() {
    let mut bad = op("bad", "AES Decrypt");
    if let NodeKind::Operation { args, .. } = &mut bad.kind {
        args.insert("Key".into(), "hex:00".into());
        args.insert("Input".into(), "Raw".into());
    }
    let g = graph(
        vec![input("in"), bad, output("bad-out"), output("good-out")],
        vec![
            edge("in", "bad", "data"),
            edge("bad", "bad-out", "data"),
            edge("in", "good-out", "data"),
        ],
    );
    let r = run(&g, &[("in", b"not hex")]);
    assert_eq!(bytes(&r, "good-out"), b"not hex");
    assert_eq!(
        r.outputs
            .iter()
            .find(|o| o.name == "bad-out")
            .unwrap()
            .error
            .as_ref()
            .unwrap()
            .kind,
        GraphErrorKind::Dependency
    );
    assert!(r
        .nodes
        .iter()
        .any(|n| n.node_id == "bad" && n.status == NodeStatus::Failed));
}
#[test]
fn disconnected_nodes_do_not_execute_and_selection_excludes_other_inputs() {
    let mut g = graph(
        vec![
            input("a"),
            input("b"),
            op("unused", "From Hex"),
            output("a-out"),
            output("b-out"),
        ],
        vec![edge("a", "a-out", "data"), edge("b", "b-out", "data")],
    );
    let r = execute(
        &g,
        BTreeMap::from([("a".into(), vec![255])]),
        &["a-out".into()],
        &GraphPolicy::default(),
        &ExecutionControl::default(),
    )
    .unwrap();
    assert_eq!(bytes(&r, "a-out"), [255]);
    assert_eq!(r.outputs.len(), 1);
    assert!(r
        .nodes
        .iter()
        .any(|n| n.node_id == "unused" && n.status == NodeStatus::Disconnected));
    g.edges.push(edge("unused", "unused", "data"));
    assert_eq!(
        validate(&g, &GraphPolicy::default()).unwrap_err().kind,
        GraphErrorKind::Cycle
    );
}
#[test]
fn rejects_duplicate_ids_unknown_nodes_invalid_ports_and_missing_connections() {
    let valid = Graph::from_recipe("valid".into(), vec![]);
    let mut cases = Vec::new();
    let mut g = valid.clone();
    g.nodes.push(g.nodes[0].clone());
    cases.push(g);
    let mut g = valid.clone();
    g.edges.push(g.edges[0].clone());
    cases.push(g);
    let mut g = valid.clone();
    g.edges[0].source_node = "unknown".into();
    cases.push(g);
    let mut g = valid.clone();
    g.edges[0].source_port = "nope".into();
    cases.push(g);
    let mut g = valid.clone();
    g.edges[0].target_port = "arg:unknown".into();
    cases.push(g);
    let mut g = valid.clone();
    g.edges.remove(0);
    cases.push(g);
    let mut g = valid.clone();
    g.nodes[0].id.clear();
    cases.push(g);
    for g in cases {
        assert!(validate(&g, &GraphPolicy::default()).is_err());
    }
}
#[test]
fn versions_and_resource_limits_are_enforced() {
    let mut g = Graph::from_recipe("valid".into(), vec![]);
    g.version = 2;
    assert_eq!(
        validate(&g, &GraphPolicy::default()).unwrap_err().kind,
        GraphErrorKind::Version
    );
    g.version = 1;
    for field in [0, 1, 2, 3] {
        let mut p = GraphPolicy::default();
        match field {
            0 => p.limits.max_nodes = 2,
            1 => p.limits.max_edges = 1,
            2 => p.limits.max_depth = 2,
            _ => p.limits.max_definition_bytes = 10,
        }
        assert_eq!(validate(&g, &p).unwrap_err().kind, GraphErrorKind::Limit);
    }
    let inputs = BTreeMap::from([("input".into(), vec![0, 255])]);
    let mut p = GraphPolicy::default();
    p.limits.max_input_bytes = 1;
    assert_eq!(
        execute(&g, inputs.clone(), &[], &p, &ExecutionControl::default())
            .unwrap_err()
            .kind,
        GraphErrorKind::Limit
    );
    p = GraphPolicy::default();
    p.limits.max_live_bytes = 1;
    assert_eq!(
        execute(&g, inputs.clone(), &[], &p, &ExecutionControl::default())
            .unwrap_err()
            .kind,
        GraphErrorKind::Limit
    );
    p = GraphPolicy::default();
    p.limits.max_output_bytes = 1;
    let result = execute(&g, inputs, &[], &p, &ExecutionControl::default()).unwrap();
    assert_eq!(
        result.outputs[0].error.as_ref().unwrap().kind,
        GraphErrorKind::OutputLimit
    );
    assert!(result.outputs[0].output_base64.is_none());
}
#[test]
fn cancellation_and_deadline_return_typed_errors() {
    let g = Graph::from_recipe("cancel".into(), vec![]);
    let cancelled = ExecutionControl {
        cancelled: Some(Arc::new(AtomicBool::new(true))),
        timeout: None,
    };
    assert_eq!(
        execute(
            &g,
            BTreeMap::new(),
            &[],
            &GraphPolicy::default(),
            &cancelled
        )
        .unwrap_err()
        .kind,
        GraphErrorKind::Cancelled
    );
    let timeout = ExecutionControl {
        timeout: Some(Duration::ZERO),
        cancelled: None,
    };
    assert_eq!(
        execute(&g, BTreeMap::new(), &[], &GraphPolicy::default(), &timeout)
            .unwrap_err()
            .kind,
        GraphErrorKind::Timeout
    );
}
#[test]
fn combine_semantics_include_strict_xor_and_utf8() {
    for (combine, left, right, expected) in [
        (Combine::Xor, vec![0, 255], vec![255, 0], vec![255, 255]),
        (Combine::Compare, vec![255], vec![255], vec![1]),
        (Combine::Compare, vec![255], vec![0], vec![0]),
        (
            Combine::Interleave,
            vec![1, 2, 3],
            vec![4],
            vec![1, 4, 2, 3],
        ),
    ] {
        let g = graph(
            vec![
                input("a"),
                input("b"),
                node("combine", NodeKind::Combine { combine }),
                output("out"),
            ],
            vec![
                edge("a", "combine", "left"),
                edge("b", "combine", "right"),
                edge("combine", "out", "data"),
            ],
        );
        assert_eq!(
            bytes(&run(&g, &[("a", &left), ("b", &right)]), "out"),
            expected
        );
    }
    let g = graph(
        vec![
            input("a"),
            input("b"),
            node(
                "xor",
                NodeKind::Combine {
                    combine: Combine::Xor,
                },
            ),
            output("out"),
        ],
        vec![
            edge("a", "xor", "left"),
            edge("b", "xor", "right"),
            edge("xor", "out", "data"),
        ],
    );
    assert!(run(&g, &[("a", &[0]), ("b", &[])]).outputs[0]
        .output_base64
        .is_none());
    let mut g = g;
    g.nodes[2].kind = NodeKind::Combine {
        combine: Combine::JoinText {
            inputs: 2,
            separator: "|".into(),
        },
    };
    g.edges[0].target_port = "input:0".into();
    g.edges[1].target_port = "input:1".into();
    assert_eq!(bytes(&run(&g, &[("a", b"a"), ("b", b"b")]), "out"), b"a|b");
    assert!(run(&g, &[("a", &[255]), ("b", b"b")]).outputs[0]
        .output_base64
        .is_none());
}
#[test]
fn separate_ciphertext_key_iv_inputs_use_the_registered_aes_operation() {
    let key = [0u8; 16];
    let iv = [1u8; 16];
    let plaintext = [0, 255, 192, 0, 65];
    let args = vec![
        format!("hex:{}", hex::encode(key)),
        format!("hex:{}", hex::encode(iv)),
        "CBC".into(),
        "Raw".into(),
        "Raw".into(),
    ];
    let ciphertext =
        crate::runtime::run_operation("AES Encrypt", plaintext.to_vec(), &args).unwrap();
    let mut aes = op("aes", "AES Decrypt");
    if let NodeKind::Operation { args, .. } = &mut aes.kind {
        args.insert("Input".into(), "Raw".into());
    }
    let g = graph(
        vec![
            input("ciphertext"),
            input("key"),
            input("iv"),
            aes,
            output("plaintext"),
        ],
        vec![
            edge("ciphertext", "aes", "data"),
            edge("key", "aes", "arg:key"),
            edge("iv", "aes", "arg:iv"),
            edge("aes", "plaintext", "data"),
        ],
    );
    assert_eq!(
        bytes(
            &run(
                &g,
                &[("ciphertext", &ciphertext), ("key", &key), ("iv", &iv)]
            ),
            "plaintext"
        ),
        plaintext
    );
    let json = serde_json::to_string(&g).unwrap();
    assert!(!json.contains(&hex::encode(key)));
}
#[test]
fn named_argument_ports_require_matching_adapters() {
    let mut hash = op("hash", "SHA2");
    let schema = crate::runtime::operation_info("SHA2").unwrap();
    let arg = &schema.args[0];
    if let NodeKind::Operation { args, .. } = &mut hash.kind {
        args.clear();
    }
    let g = graph(
        vec![
            input("data"),
            input("arg"),
            node("adapter", NodeKind::Argument { kind: arg.kind }),
            hash,
            output("out"),
        ],
        vec![
            edge("data", "hash", "data"),
            edge("arg", "adapter", "data"),
            edge(
                "adapter",
                "hash",
                &format!("arg:{}", crate::runtime::canonical_identifier(arg.name)),
            ),
            edge("hash", "out", "data"),
        ],
    );
    let r = run(&g, &[("data", b"hello"), ("arg", b"256")]);
    assert_eq!(
        bytes(&r, "out"),
        crate::runtime::run_operation("SHA2", b"hello".to_vec(), &["256".into()]).unwrap()
    );
    let mut wrong = g;
    wrong.edges[2].source_node = "arg".into();
    assert_eq!(
        validate(&wrong, &GraphPolicy::default()).unwrap_err().kind,
        GraphErrorKind::InvalidPort
    );
}
#[test]
fn embedded_recipe_retains_fork_merge_and_policy() {
    let steps = vec![
        crate::execution::RecipeStep {
            op: "Fork".into(),
            args: vec!["\\n".into(), "|".into(), "false".into()],
        },
        crate::execution::RecipeStep {
            op: "To Upper case".into(),
            args: vec![],
        },
        crate::execution::RecipeStep {
            op: "Merge".into(),
            args: vec![],
        },
    ];
    let g = Graph::from_recipe("fork".into(), steps);
    assert_eq!(
        bytes(&run(&g, &[("input", b"one\ntwo")]), "output"),
        b"ONE|TWO"
    );
    let g = graph(vec![op("fork", "Fork")], vec![]);
    assert!(validate(&g, &GraphPolicy::default()).is_err());
    let g = graph(vec![op("network", "HTTP request")], vec![]);
    assert_eq!(
        validate(&g, &GraphPolicy::default()).unwrap_err().kind,
        GraphErrorKind::PolicyDenied
    );
    let g = Graph::from_recipe(
        "denied".into(),
        vec![crate::execution::RecipeStep {
            op: "HTTP request".into(),
            args: vec![],
        }],
    );
    assert_eq!(
        validate(&g, &GraphPolicy::default()).unwrap_err().kind,
        GraphErrorKind::PolicyDenied
    );
}

#[test]
fn conversion_never_discards_branches_or_detached_nodes() {
    let g = Graph::from_recipe(
        "recipe".into(),
        vec![crate::execution::RecipeStep {
            op: "To Base64".into(),
            args: vec![],
        }],
    );
    assert_eq!(
        g.to_recipe(&GraphPolicy::default()).unwrap()[0].op,
        "To Base64"
    );
    assert!(binary_graph().to_recipe(&GraphPolicy::default()).is_err());
    let mut g = g;
    g.nodes.push(input("detached"));
    assert!(g.to_recipe(&GraphPolicy::default()).is_err());
}

#[test]
fn file_references_and_unknown_node_fields_are_rejected() {
    let g = Graph::from_recipe("safe".into(), vec![]);
    let mut value = serde_json::to_value(g).unwrap();
    value["nodes"][0]["path"] = serde_json::json!("/etc/passwd");
    assert!(Graph::from_json(
        &serde_json::to_vec(&value).unwrap(),
        &GraphPolicy::default()
    )
    .is_err());
}
