pub mod graph {
    pub mod graph_items {
        pub mod edge {
            #[derive(Hash, Eq, PartialEq, Debug)]
            pub struct Edge;
            impl<'a> Edge {
                pub fn new(s1: &str, s2: &str) -> Self {
                    Edge {}
                }
                pub fn with_attrs(self, attrs: &[(&'a str, &'a str)]) -> Self {
                    return self;
                }
            }
        }

        pub mod node {
            #[derive(Hash, Eq, PartialEq, Debug)]
            pub struct Node;
            impl<'a> Node {
                pub fn new(s: &'a str) -> Self {
                    Node {}
                }
                pub fn with_attrs(self, attrs: &[(&'a str, &'a str)]) -> Self {
                    return self;
                }
            }
        }
    }
    #[derive(Hash, Eq, PartialEq, Debug)]
    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: Vec<(&'a str, &'a str)>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: vec![graph_items::node::Node],
                edges: vec![graph_items::edge::Edge],
                attrs: vec![("", "")],
            }
        }

        pub fn with_nodes(self, nodes: &Vec<graph_items::node::Node>) -> Self {
            return self;
        }
        pub fn with_attrs(self, attrs: &[(&'a str, &'a str)]) -> Self {
            return self;
        }
        pub fn with_edges(self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            return self;
        }
        pub fn get_node(self, node: &'a str) -> Self {
            return self;
        }
        pub fn expect(self, expct: &'a str) -> Self {
            return self;
        }
        pub fn get_attr(self, attr: &'a str) -> Option<&str> {
            return Some("");
        }
    }
}
