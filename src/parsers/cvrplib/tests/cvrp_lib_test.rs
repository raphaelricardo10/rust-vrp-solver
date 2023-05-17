use std::fs;

use rstest::rstest;

use crate::parsers::{
    cvrplib::{
        cvrplib_parser::CvrpLibParser,
        sections::{Demand, Header, Node},
    },
    vrp_parser::VrpParser,
};

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
    match fs::read_to_string("./src/parsers/cvrplib/tests/A-n32-k5.vrp") {
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
