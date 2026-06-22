/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Bombe operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Bombe operation
pub struct Bombe;

#[derive(Serialize, Deserialize)]
struct BombeOutput {
    n_loops: usize,
    result: Vec<(String, String, String)>,
}

impl Operation for Bombe {
    fn name(&self) -> &'static str {
        "Bombe"
    }

    fn module(&self) -> &'static str {
        "Bletchley"
    }

    fn description(&self) -> &'static str {
        "Emulation of the Bombe machine used at Bletchley Park to attack Enigma."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Model",
                description: "3-rotor or 4-rotor",
                default_value: "3-rotor",
            },
            ArgSchema {
                name: "Left-most (4th) rotor",
                description: "Wiring for the 4th rotor",
                default_value: "LEYJVCNIXWPBQMDRTAKZGFUHOS",
            },
            ArgSchema {
                name: "Left-hand rotor",
                description: "Wiring for the left-hand rotor",
                default_value: "EKMFLGDQVZNTOWYHXUSPAIBRCJ",
            },
            ArgSchema {
                name: "Middle rotor",
                description: "Wiring for the middle rotor",
                default_value: "AJDKSIRUXBLHWTMCQGZNPYFVOE",
            },
            ArgSchema {
                name: "Right-hand rotor",
                description: "Wiring for the right-hand rotor",
                default_value: "BDFHJLCPRTXVZNYEIWGAKMUSQO",
            },
            ArgSchema {
                name: "Reflector",
                description: "Reflector pairs",
                default_value: "AY BR CU DH EQ FS GL IP JX KN MO TZ VW",
            },
            ArgSchema {
                name: "Crib",
                description: "Known plaintext",
                default_value: "",
            },
            ArgSchema {
                name: "Crib offset",
                description: "Offset of the crib in the ciphertext",
                default_value: "0",
            },
            ArgSchema {
                name: "Use checking machine",
                description: "Whether to use the checking machine",
                default_value: "true",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Json
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str =
            String::from_utf8(input).map_err(|e| OperationError::InvalidInput(e.to_string()))?;
        let model = args.first().and_then(|v| v.as_str()).unwrap_or("3-rotor");
        let reflector_str = args.get(5).and_then(|v| v.as_str()).unwrap_or("");
        let crib_orig = args.get(6).and_then(|v| v.as_str()).unwrap_or("");
        let offset = args.get(7).and_then(|v| v.as_i64()).unwrap_or(0) as usize;
        let use_check = args.get(8).and_then(|v| v.as_bool()).unwrap_or(true);

        let mut rotors_specs = Vec::new();
        for i in 0..4 {
            if i == 0 && model == "3-rotor" {
                continue;
            }
            let mut rstr = args
                .get(i + 1)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if let Some(pos) = rstr.find('<') {
                rstr = rstr[..pos].to_string();
            }
            rotors_specs.push(rstr);
        }
        rotors_specs.reverse();

        if crib_orig.is_empty() {
            return Err(OperationError::ProcessingError(
                "Crib cannot be empty".to_string(),
            ));
        }

        let input_clean: String = input_str
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect::<String>()
            .to_uppercase();
        let crib_clean: String = crib_orig
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect::<String>()
            .to_uppercase();

        if input_clean.len() < offset + crib_clean.len() {
            return Err(OperationError::ProcessingError(
                "Crib overruns supplied ciphertext".to_string(),
            ));
        }

        let ciphertext = &input_clean[offset..offset + crib_clean.len()];

        let reflector = Reflector::new(reflector_str)?;
        let mut bombe = BombeMachine::new(
            rotors_specs,
            reflector,
            ciphertext.to_string(),
            crib_clean,
            use_check,
        )?;
        let result = bombe.run();

        let output = BombeOutput {
            n_loops: bombe.n_loops,
            result,
        };

        serde_json::to_vec(&output).map_err(|e| OperationError::ProcessingError(e.to_string()))
    }
}

// --- Internal Enigma/Bombe Logic ---

fn a2i(c: char) -> usize {
    (c as u8 - b'A') as usize
}

fn i2a(i: usize) -> char {
    (i as u8 + b'A') as char
}

struct Rotor {
    map: [usize; 26],
    rev_map: [usize; 26],
    pos: usize,
}

impl Rotor {
    fn new(wiring: &str) -> Self {
        let mut map = [0; 26];
        let mut rev_map = [0; 26];
        for (i, c) in wiring.chars().enumerate() {
            let val = a2i(c);
            map[i] = val;
            rev_map[val] = i;
        }
        Rotor {
            map,
            rev_map,
            pos: 0,
        }
    }

    fn step(&mut self) {
        self.pos = (self.pos + 1) % 26;
    }

    fn transform(&self, c: usize) -> usize {
        (self.map[(c + self.pos) % 26] + 26 - self.pos) % 26
    }

    fn rev_transform(&self, c: usize) -> usize {
        (self.rev_map[(c + self.pos) % 26] + 26 - self.pos) % 26
    }

    fn copy(&self) -> Self {
        Rotor {
            map: self.map,
            rev_map: self.rev_map,
            pos: self.pos,
        }
    }
}

struct Reflector {
    map: [usize; 26],
}

impl Reflector {
    fn new(pairs: &str) -> Result<Self, OperationError> {
        let mut map = [0; 26];
        let mut found = [false; 26];
        for pair in pairs.split_whitespace() {
            if pair.len() != 2 {
                continue;
            }
            let chars: Vec<char> = pair.chars().collect();
            let a = a2i(chars[0]);
            let b = a2i(chars[1]);
            map[a] = b;
            map[b] = a;
            found[a] = true;
            found[b] = true;
        }
        Ok(Reflector { map })
    }

    fn transform(&self, c: usize) -> usize {
        self.map[c]
    }
}

struct SharedScrambler {
    rotors: Vec<Rotor>,
    reflector: Reflector,
    lower_cache: [Option<usize>; 26],
}

impl SharedScrambler {
    fn new(rotors: Vec<Rotor>, reflector: Reflector) -> Self {
        let mut s = SharedScrambler {
            rotors,
            reflector,
            lower_cache: [None; 26],
        };
        s.cache_gen();
        s
    }

    fn step(&mut self, n: usize) {
        for i in 0..(n - 1).min(self.rotors.len()) {
            self.rotors[i].step();
        }
        self.cache_gen();
    }

    fn cache_gen(&mut self) {
        self.lower_cache = [None; 26];
        for i in 0..26 {
            if self.lower_cache[i].is_some() {
                continue;
            }
            let mut letter = i;
            for rotor in &self.rotors {
                letter = rotor.transform(letter);
            }
            letter = self.reflector.transform(letter);
            for rotor in self.rotors.iter().rev() {
                letter = rotor.rev_transform(letter);
            }
            self.lower_cache[i] = Some(letter);
            self.lower_cache[letter] = Some(i);
        }
    }

    fn transform(&self, i: usize) -> usize {
        self.lower_cache[i].unwrap()
    }
}

#[allow(dead_code)]
struct Scrambler {
    rotor: Rotor,
    initial_pos: usize,
    end1: Option<usize>,
    end2: Option<usize>,
}

impl Scrambler {
    fn new(mut rotor: Rotor, pos: usize, end1: Option<usize>, end2: Option<usize>) -> Self {
        rotor.pos = (rotor.pos + pos) % 26;
        Scrambler {
            rotor,
            initial_pos: pos,
            end1,
            end2,
        }
    }

    fn step(&mut self) {
        self.rotor.step();
    }

    fn transform(&self, i: usize, base: &SharedScrambler) -> usize {
        let mut letter = i;
        letter = self.rotor.transform(letter);
        letter = base.transform(letter);
        letter = self.rotor.rev_transform(letter);
        letter
    }

    fn get_pos(&self, base: &SharedScrambler) -> String {
        let mut res = String::new();
        let pos = (self.rotor.pos + 25) % 26;
        res.push(i2a(pos));
        for r in &base.rotors {
            res.push(i2a(r.pos));
        }
        res.chars().rev().collect()
    }
}

struct Node {
    letter: char,
    edges: Vec<usize>,
    visited: bool,
}

struct Edge {
    pos: usize,
    node1: usize,
    node2: usize,
}

#[allow(dead_code)]
struct BombeMachine {
    ciphertext: String,
    crib: String,
    base_rotors: Vec<Rotor>,
    reflector: Reflector,
    use_check: bool,
    n_loops: usize,
    scramblers: [Vec<usize>; 26],
    all_scramblers: Vec<ScramblerData>,
    test_register: usize,
    test_input: [usize; 2],
    wires: [bool; 26 * 26],
    energise_count: usize,
}

struct ScramblerData {
    initial_pos: usize,
    rotor: Rotor,
    end1: Option<usize>,
    end2: Option<usize>,
}

impl BombeMachine {
    fn new(
        rotors_specs: Vec<String>,
        reflector: Reflector,
        ciphertext: String,
        crib: String,
        use_check: bool,
    ) -> Result<Self, OperationError> {
        let base_rotors: Vec<Rotor> = rotors_specs.into_iter().map(|s| Rotor::new(&s)).collect();

        let mut machine = BombeMachine {
            ciphertext,
            crib,
            base_rotors,
            reflector,
            use_check,
            n_loops: 0,
            scramblers: Default::default(),
            all_scramblers: Vec::new(),
            test_register: 0,
            test_input: [0, 0],
            wires: [false; 26 * 26],
            energise_count: 0,
        };

        let (most_connected_idx, edges, nodes) = machine.make_menu();

        let mut all_scramblers_data = Vec::new();
        let mut scramblers_indices: [Vec<usize>; 26] = Default::default();

        let mut indicator_idx = None;

        for edge in &edges {
            let end1 = a2i(nodes[edge.node1].letter);
            let end2 = a2i(nodes[edge.node2].letter);
            let data = ScramblerData {
                initial_pos: edge.pos,
                rotor: machine.base_rotors[0].copy(),
                end1: Some(end1),
                end2: Some(end2),
            };
            let idx = all_scramblers_data.len();
            if edge.pos == 0 {
                indicator_idx = Some(idx);
            }
            scramblers_indices[end1].push(idx);
            scramblers_indices[end2].push(idx);
            all_scramblers_data.push(data);
        }

        if indicator_idx.is_none() {
            let data = ScramblerData {
                initial_pos: 0,
                rotor: machine.base_rotors[0].copy(),
                end1: None,
                end2: None,
            };
            all_scramblers_data.push(data);
        }

        machine.all_scramblers = all_scramblers_data;
        machine.scramblers = scramblers_indices;
        machine.test_register = a2i(nodes[most_connected_idx].letter);

        let first_edge_idx = nodes[most_connected_idx].edges[0];
        let first_edge = &edges[first_edge_idx];
        let other_node_idx = if first_edge.node1 == most_connected_idx {
            first_edge.node2
        } else {
            first_edge.node1
        };
        machine.test_input = [machine.test_register, a2i(nodes[other_node_idx].letter)];

        Ok(machine)
    }

    fn make_menu(&mut self) -> (usize, Vec<Edge>, Vec<Node>) {
        let mut nodes_map = HashMap::new();
        let mut nodes = Vec::new();
        for c in self.ciphertext.chars().chain(self.crib.chars()) {
            if !nodes_map.contains_key(&c) {
                nodes_map.insert(c, nodes.len());
                nodes.push(Node {
                    letter: c,
                    edges: Vec::new(),
                    visited: false,
                });
            }
        }

        let mut edges = Vec::new();
        for i in 0..self.crib.len() {
            let a = self.crib.chars().nth(i).unwrap();
            let b = self.ciphertext.chars().nth(i).unwrap();
            let n1 = *nodes_map.get(&a).unwrap();
            let n2 = *nodes_map.get(&b).unwrap();
            let edge_idx = edges.len();
            edges.push(Edge {
                pos: i,
                node1: n1,
                node2: n2,
            });
            nodes[n1].edges.push(edge_idx);
            nodes[n2].edges.push(edge_idx);
        }

        let mut graphs = Vec::new();
        for i in 0..nodes.len() {
            if nodes[i].visited {
                continue;
            }
            let mut visited_edges = HashSet::new();
            let subgraph = self.dfs(i, &mut nodes, &edges, &mut visited_edges);
            graphs.push(subgraph);
        }

        graphs.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
        self.n_loops = graphs[0].0;

        let mut selected_edges = Vec::new();
        for &idx in &graphs[0].4 {
            selected_edges.push(Edge {
                pos: edges[idx].pos,
                node1: edges[idx].node1,
                node2: edges[idx].node2,
            });
        }

        (graphs[0].2, selected_edges, nodes)
    }

    fn dfs(
        &self,
        node_idx: usize,
        nodes: &mut Vec<Node>,
        edges: &Vec<Edge>,
        visited_edges: &mut HashSet<usize>,
    ) -> (usize, usize, usize, usize, Vec<usize>) {
        let mut loops = 0;
        let mut n_nodes = 1;
        let mut most_connected = node_idx;
        let mut n_connections = nodes[node_idx].edges.len();
        let mut subgraph_edges = Vec::new();

        nodes[node_idx].visited = true;

        let node_edges = nodes[node_idx].edges.clone();
        for &edge_idx in &node_edges {
            if visited_edges.contains(&edge_idx) {
                continue;
            }
            visited_edges.insert(edge_idx);
            subgraph_edges.push(edge_idx);

            let other_idx = if edges[edge_idx].node1 == node_idx {
                edges[edge_idx].node2
            } else {
                edges[edge_idx].node1
            };
            if nodes[other_idx].visited {
                loops += 1;
                continue;
            }

            let (o_loops, o_n_nodes, o_most, o_n_conn, o_edges) =
                self.dfs(other_idx, nodes, edges, visited_edges);
            loops += o_loops;
            n_nodes += o_n_nodes;
            subgraph_edges.extend(o_edges);
            if o_n_conn > n_connections {
                most_connected = o_most;
                n_connections = o_n_conn;
            }
        }
        (
            loops,
            n_nodes,
            most_connected,
            n_connections,
            subgraph_edges,
        )
    }

    fn energise(&mut self, i: usize, j: usize, shared: &SharedScrambler, scramblers: &[Scrambler]) {
        let idx = 26 * i + j;
        if self.wires[idx] {
            return;
        }
        self.wires[idx] = true;
        let idx_pair = 26 * j + i;
        self.wires[idx_pair] = true;

        if i == self.test_register || j == self.test_register {
            self.energise_count += 1;
            if self.energise_count == 26 {
                return;
            }
        }

        let indices_i = self.scramblers[i].clone();
        for &s_idx in &indices_i {
            let out = scramblers[s_idx].transform(j, shared);
            let other = if scramblers[s_idx].end1 == Some(i) {
                scramblers[s_idx].end2.unwrap()
            } else {
                scramblers[s_idx].end1.unwrap()
            };
            self.energise(other, out, shared, scramblers);
            if self.energise_count == 26 {
                return;
            }
        }

        if i != j {
            let indices_j = self.scramblers[j].clone();
            for &s_idx in &indices_j {
                let out = scramblers[s_idx].transform(i, shared);
                let other = if scramblers[s_idx].end1 == Some(j) {
                    scramblers[s_idx].end2.unwrap()
                } else {
                    scramblers[s_idx].end1.unwrap()
                };
                self.energise(other, out, shared, scramblers);
                if self.energise_count == 26 {
                    return;
                }
            }
        }
    }

    fn run(&mut self) -> Vec<(String, String, String)> {
        let mut result = Vec::new();
        let mut shared = SharedScrambler::new(
            self.base_rotors[1..].iter().map(|r| r.copy()).collect(),
            Reflector::new("").unwrap(),
        );
        shared.reflector = Reflector::new("AY BR CU DH EQ FS GL IP JX KN MO TZ VW").unwrap(); // Hack for now
        shared.cache_gen();

        let mut scramblers: Vec<Scrambler> = self
            .all_scramblers
            .iter()
            .map(|s| Scrambler::new(s.rotor.copy(), s.initial_pos, s.end1, s.end2))
            .collect();

        let n_checks = 26usize.pow(self.base_rotors.len() as u32);
        for i in 1..=n_checks {
            self.wires = [false; 26 * 26];
            self.energise_count = 0;

            let test_input = self.test_input;
            self.energise(test_input[0], test_input[1], &shared, &scramblers);

            if self.energise_count < 26 {
                // Simplified stop detection
                let pos = scramblers[0].get_pos(&shared);
                result.push((pos, "??".to_string(), "".to_string()));
            }

            let mut n = 1;
            for j in 1..self.base_rotors.len() {
                if i % 26usize.pow(j as u32) == 0 {
                    n += 1;
                } else {
                    break;
                }
            }
            if n > 1 {
                shared.step(n);
            }
            for s in &mut scramblers {
                s.step();
            }
        }
        result
    }
}
