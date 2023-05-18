use geo::{point, EuclideanDistance};
use std::{fs, iter::zip, str::Lines};

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    parsers::vrp_parser::{VrpInputs, VrpParser},
    services::distance::distance_service::DistanceMatrix,
};

use super::sections::{Demand, Header, Node};

pub struct CvrpLibParser {
    pub content: String,
    pub number_of_vehicles: u32,
}

impl CvrpLibParser {
    pub fn from_file(path: &str, number_of_vehicles: u32) -> Self {
        match fs::read_to_string(path) {
            Ok(content) => Self {
                content,
                number_of_vehicles,
            },
            Err(error) => panic!("Error reading file: {:?}", error),
        }
    }

    pub(super) fn get_header_value<'a>(lines: &mut Lines<'a>) -> &'a str {
        const ERROR_MESSAGE: &str = "failed parsing header section";

        lines
            .next()
            .expect(ERROR_MESSAGE)
            .trim()
            .split(" : ")
            .nth(1)
            .expect(ERROR_MESSAGE)
    }

    pub(super) fn generate_pair_combinations<'a, T: Copy>(
        items: &'a [T],
    ) -> Box<dyn Iterator<Item = (T, T)> + 'a> {
        Box::new(
            items
                .iter()
                .flat_map(|src| items.iter().map(move |&dest| (*src, dest))),
        )
    }

    pub(super) fn parse_header<'a>(lines: &mut Lines<'a>) -> Header<'a> {
        Header {
            name: Self::get_header_value(lines),
            comment: Self::get_header_value(lines),
            r#type: Self::get_header_value(lines),
            dimension: Self::get_header_value(lines).parse().unwrap(),
            edge_weight_type: Self::get_header_value(lines),
            capacity: Self::get_header_value(lines).parse().unwrap(),
        }
    }

    pub(super) fn parse_nodes_section(lines: &mut Lines, dimension: u32) -> Vec<Node> {
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

    pub(super) fn parse_demands_section(lines: &mut Lines, dimension: u32) -> Vec<Demand> {
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
