use geo::{point, EuclideanDistance};
use rstest::rstest;
use std::{iter::zip, str::Lines, fs};

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::distance::distance_service::DistanceMatrix,
};

use super::vrp_parser::{VrpInputs, VrpParser};

pub(crate) struct CvrpLibParser {
    content: String,
    number_of_vehicles: u32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Node {
    id: u32,
    x_position: i16,
    y_position: i16,
}

impl From<(&str, &str, &str)> for Node {
    fn from((id, x_position, y_position): (&str, &str, &str)) -> Self {
        Self {
            id: id.parse().unwrap(),
            x_position: x_position.parse().unwrap(),
            y_position: y_position.parse().unwrap(),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Header<'a> {
    name: &'a str,
    comment: &'a str,
    r#type: &'a str,
    dimension: u32,
    edge_weight_type: &'a str,
    capacity: u32,
}

#[derive(PartialEq, Debug)]
struct Demand {
    node_id: u32,
    demand: u32,
}

impl From<(&str, &str)> for Demand {
    fn from((node_id, demand): (&str, &str)) -> Self {
        Self {
            node_id: node_id.parse().unwrap(),
            demand: demand.parse().unwrap(),
        }
    }
}

impl CvrpLibParser {
    fn get_header_value<'a>(lines: &mut Lines<'a>) -> &'a str {
        const ERROR_MESSAGE: &str = "failed parsing header section";

        let result = lines
            .next()
            .expect(ERROR_MESSAGE)
            .trim()
            .split(" : ")
            .nth(1)
            .expect(ERROR_MESSAGE);

        print!("{}", result);
        result
    }

    fn generate_pair_combinations<'a, T: Copy>(
        items: &'a [T],
    ) -> Box<dyn Iterator<Item = (T, T)> + 'a> {
        Box::new(
            items
                .iter()
                .flat_map(|src| items.iter().map(move |&dest| (*src, dest))),
        )
    }

    fn parse_header<'a>(lines: &mut Lines<'a>) -> Header<'a> {
        Header {
            name: Self::get_header_value(lines),
            comment: Self::get_header_value(lines),
            r#type: Self::get_header_value(lines),
            dimension: Self::get_header_value(lines).parse().unwrap(),
            edge_weight_type: Self::get_header_value(lines),
            capacity: Self::get_header_value(lines).parse().unwrap(),
        }
    }

    fn parse_nodes_section(lines: &mut Lines, dimension: u32) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();

        for _ in 0..dimension {
            let current_line = lines.next();

            if current_line.is_none() {
                break;
            }

            let entries: Vec<&str> = current_line.unwrap().trim().split(' ').collect();
            let node = Node::from((entries[0], entries[1], entries[2]));

            nodes.push(node)
        }

        nodes
    }

    fn parse_demands_section(lines: &mut Lines, dimension: u32) -> Vec<Demand> {
        let mut demands: Vec<Demand> = Vec::new();

        for _ in 0..dimension {
            let current_line = lines.next();

            if current_line.is_none() {
                break;
            }

            let entries: Vec<&str> = current_line.unwrap().trim().split(' ').collect();
            let demand = Demand::from((entries[0], entries[1]));

            demands.push(demand)
        }

        demands
    }
}

impl VrpParser for CvrpLibParser {
    fn parse(&self) -> VrpInputs {
        let mut lines = self.content.lines();

        let header = Self::parse_header(&mut lines);
        lines.next();

        let nodes = Self::parse_nodes_section(&mut lines, header.dimension);
        lines.next();

        let demands = Self::parse_demands_section(&mut lines, header.dimension);
        lines.next();

        let stops: Vec<Stop> = zip(&nodes, demands)
            .map(|(node, demand)| Stop {
                id: node.id,
                usage: demand.demand,
            })
            .collect();

        let vehicles: Vec<Vehicle> = (0..self.number_of_vehicles)
            .map(|id| Vehicle::new(id, header.capacity))
            .collect();

        let distances: DistanceMatrix = Self::generate_pair_combinations(&nodes)
            .map(|(src, dest)| {
                let src_point = point!(x: f32::from(src.x_position), y: f32::from(src.y_position));
                let dest_point =
                    point!(x: f32::from(dest.x_position), y: f32::from(dest.y_position));

                ((src.id, dest.id), src_point.euclidean_distance(&dest_point))
            })
            .collect();

        VrpInputs {
            stops,
            vehicles,
            distances,
        }
    }
}

#[rstest]
fn test_can_parse_nodes_section() {
    let content = " 1 82 76\n 2 10 15";
    let mut lines = content.lines();
    let nodes = CvrpLibParser::parse_nodes_section(&mut lines, 2);

    assert_eq!(
        nodes[0],
        Node {
            id: 1,
            x_position: 82,
            y_position: 76
        }
    );

    assert_eq!(
        nodes[1],
        Node {
            id: 2,
            x_position: 10,
            y_position: 15
        }
    )
}

#[rstest]
fn test_can_parse_header() {
    let content = "NAME : CMT1\nCOMMENT : 524.61\nTYPE : CVRP\nDIMENSION : 51\nEDGE_WEIGHT_TYPE : EUC_2D\nCAPACITY : 160";
    let mut lines = content.lines();
    let header = CvrpLibParser::parse_header(&mut lines);

    assert_eq!(
        header,
        Header {
            name: "CMT1",
            comment: "524.61",
            r#type: "CVRP",
            dimension: 51,
            edge_weight_type: "EUC_2D",
            capacity: 160
        }
    )
}

#[rstest]
fn test_can_parse_demands_section() {
    let content = "1 0  \n2 19  \n3 21   ";
    let mut lines = content.lines();
    let demands = CvrpLibParser::parse_demands_section(&mut lines, 3);

    assert_eq!(
        demands[0],
        Demand {
            node_id: 1,
            demand: 0
        }
    );

    assert_eq!(
        demands[1],
        Demand {
            node_id: 2,
            demand: 19
        }
    );

    assert_eq!(
        demands[2],
        Demand {
            node_id: 3,
            demand: 21
        }
    );
}

#[rstest]
fn test_can_generate_all_pair_combinations() {
    let nodes = vec![1, 2];
    let combinations: Vec<(u32, u32)> = CvrpLibParser::generate_pair_combinations(&nodes).collect();
    let expected_result = vec![(1, 1), (1, 2), (2, 1), (2, 2)];

    assert_eq!(combinations, expected_result);
}

#[rstest]
fn test_parse_the_whole_file() {
    match fs::read_to_string("./src/parsers/A-n32-k5.vrp") {
        Ok(content) => {
            let vrp_inputs = CvrpLibParser {
                content,
                number_of_vehicles: 5,
            }
            .parse();

            assert_eq!(vrp_inputs.stops.len(), 32);
            assert_eq!(vrp_inputs.vehicles.len(), 5);
            assert_eq!(vrp_inputs.distances.len(), 1024);
        }
        Err(error) => panic!("Error reading file: {:?}", error),
    }
}
