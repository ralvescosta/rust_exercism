pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            #[derive(Clone, Eq, PartialEq, Debug)]
            pub struct Edge;
            impl Edge {
                pub fn new(s1: &str, s2: &str) -> Self {
                    Edge {}
                }
                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    return self;
                }
            }
        }

        pub mod node {
            #[derive(Clone, Eq, PartialEq, Debug)]
            pub struct Node;
            impl Node {
                pub fn new(s: &str) -> Self {
                    Node {}
                }
                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    return self;
                }
            }
        }
    }
    #[derive(Clone, Eq, PartialEq, Debug)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![graph_items::node::Node],
                edges: vec![graph_items::edge::Edge],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &Vec<graph_items::node::Node>) -> Self {
            return self;
        }
        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            return self;
        }
        pub fn with_edges(self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            return self;
        }
        pub fn get_node(self, node: &str) -> Self {
            return self;
        }
        pub fn expect(self, expct: &str) -> Self {
            return self;
        }
        pub fn get_attr(self, attr: &str) -> Option<&str> {
            return Some("");
        }
    }
}
