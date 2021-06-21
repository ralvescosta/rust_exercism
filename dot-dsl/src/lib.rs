pub mod graph {
    pub mod graph_items {
        pub mod edge {
            #[derive(Debug)]
            pub struct Edge;
            impl Edge {
                pub fn new(s1: &str, s2: &str) -> Self {
                    Edge {}
                }
            }
        }

        pub mod node {
            #[derive(Debug)]
            pub struct Node;
            impl Node {
                pub fn new(s: &str) -> Self {
                    Node {}
                }
            }
        }
    }
    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: Vec<(&'a str, &'a str)>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            unimplemented!("Construct a new Graph struct.");
        }
        pub fn with_nodes(&self, nodes: &graph_items::node::Node) -> &Self {
            return self;
        }
    }
}
