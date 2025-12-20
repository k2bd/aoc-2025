use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Add,
};

use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

#[pymodule(module = "aoc_2026.rs.day08")]
pub fn day8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Network>()?;

    Ok(())
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day08")]
#[derive(Hash, PartialEq, Eq, PartialOrd, Clone, Copy, Debug)]
struct Coordinate {
    #[pyo3(get)]
    x: isize,
    #[pyo3(get)]
    y: isize,
    #[pyo3(get)]
    z: isize,
}

impl Add<Coordinate> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(module = "aoc_2025.rs.day08")]
struct Network {
    nodes: HashSet<Coordinate>,
    connections: HashSet<(Coordinate, Coordinate)>,

    _squared_costs: HashMap<(Coordinate, Coordinate), usize>,
}

impl From<&str> for Network {
    fn from(value: &str) -> Self {
        let mut network = Self {
            nodes: value
                .lines()
                .map(|line| {
                    let mut parts = line.split(",").map(|v| v.parse::<isize>().unwrap());
                    Coordinate {
                        x: parts.next().unwrap(),
                        y: parts.next().unwrap(),
                        z: parts.next().unwrap(),
                    }
                })
                .collect(),
            connections: HashSet::new(),
            _squared_costs: HashMap::new(),
        };
        network.generate_costs();

        network
    }
}

impl Network {
    fn connect(&mut self, from: Coordinate, to: Coordinate) {
        if !self.nodes.contains(&from) || !self.nodes.contains(&to) {
            panic!("Nodes not in network");
        };
        self.connections.insert((from, to));
        self.connections.insert((to, from));
    }

    fn generate_costs(&mut self) {
        self.nodes
            .iter()
            .flat_map(|&first_node| {
                self.nodes
                    .iter()
                    .map(move |&second_node| (first_node, second_node))
            })
            .filter(|(l, r)| l < r)
            .for_each(|(l, r)| {
                let sq_cost =
                    ((l.x - r.x).pow(2) + (l.y - r.y).pow(2) + (l.z - r.z).pow(2)) as usize;

                self._squared_costs.insert((l, r), sq_cost);
                self._squared_costs.insert((r, l), sq_cost);
            });
    }

    fn get_neighbors(&self, node: Coordinate) -> HashSet<Coordinate> {
        self.connections
            .iter()
            .filter_map(|&(from, to)| if from == node { Some(to) } else { None })
            .collect()
    }

    fn get_circuit(&self, node: Coordinate) -> HashSet<Coordinate> {
        if !self.nodes.contains(&node) {
            panic!("Node not in network");
        }

        let mut visited = HashSet::from([node]);
        let mut to_visit = self.get_neighbors(node);

        loop {
            if to_visit.is_empty() {
                break;
            }

            let mut new_visit = HashSet::<Coordinate>::new();
            to_visit.iter().for_each(|&n| {
                visited.insert(n);
                self.get_neighbors(n).into_iter().for_each(|nn| {
                    if !visited.contains(&nn) {
                        new_visit.insert(nn);
                    }
                });
            });

            to_visit = new_visit;
        }

        visited
    }

    /// Get available connections, sorted by distance ascending
    fn get_available_connections(&self) -> Vec<(Coordinate, Coordinate)> {
        let mut available_connections = self
            ._squared_costs
            .clone()
            .into_iter()
            .filter(|((from, to), _)| from < to)
            .filter(|(coord, _)| !self.connections.contains(coord))
            .collect::<Vec<_>>();
        available_connections.sort_by(|(_, a), (_, b)| a.cmp(b));

        available_connections
            .into_iter()
            .map(|(pair, _)| pair)
            .collect()
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Network {
    #[new]
    fn new(value: &str) -> Self {
        Self::from(value)
    }

    /// Get all connected circuits in the network, sorted in descending size order
    fn get_circuits(&self) -> Vec<HashSet<Coordinate>> {
        let mut nodes = self.nodes.clone();
        let mut circuits = Vec::new();

        while let Some(node) = nodes.iter().next().cloned() {
            let new_circuit = self.get_circuit(node);
            new_circuit.iter().for_each(|n| {
                nodes.remove(n);
            });
            circuits.push(new_circuit);
        }

        circuits.sort_by_key(|b| std::cmp::Reverse(b.len()));

        circuits
    }

    /// Make the best intercircuit connection, returning the coordinate added
    fn make_intercircuit_connection(&mut self) -> (Coordinate, Coordinate) {
        let circuits = self.get_circuits();
        let (from, to) = self
            .get_available_connections()
            .into_iter()
            .find(|&(from, to)| {
                !circuits
                    .iter()
                    .find(|c| c.contains(&from))
                    .unwrap()
                    .contains(&to)
            })
            .unwrap();

        self.connect(from, to);
        (from, to)
    }

    /// Make a number of connections, returning the circuits within the network
    fn make_connections(&mut self, count: usize) {
        self.get_available_connections()
            .into_iter()
            .take(count)
            .for_each(|(from, to)| {
                self.connect(from, to);
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_get_circuit() {
        let input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";
        let mut network = Network::from(input);

        network.make_connections(10);

        let circuits = network.get_circuits();
        let sizes = circuits.iter().map(|c| c.len()).collect::<Vec<_>>();

        assert_eq!(sizes.into_iter().take(3).collect::<Vec<_>>(), vec![5, 4, 2]);
    }
}
