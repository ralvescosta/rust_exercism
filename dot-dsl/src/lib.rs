pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Eq, PartialEq, Debug, Default)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(s1: &str, s2: &str) -> Self {
                    Edge {
                        from: s1.to_string(),
                        to: s2.to_string(),
                        ..Edge::default()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect();
                    return self;
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Eq, PartialEq, Debug, Default)]
            pub struct Node {
                pub node: String,
                pub attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(s: &str) -> Self {
                    Node {
                        node: s.to_string(),
                        ..Node::default()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect();
                    return self;
                }

                pub fn get_attr(&self, attr_key: &str) -> Option<&str> {
                    let attr_value = self.attrs.iter().find(|&a| a.0.as_str() == attr_key);

                    match attr_value {
                        None => Some(""),
                        Some(a) => Some(a.1.as_str()),
                    }
                }
            }
        }
    }
    #[derive(Clone, Eq, PartialEq, Debug, Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.to_vec();
            return self;
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect();
            return self;
        }
        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.to_vec();
            return self;
        }
        pub fn get_node(&self, node: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.node == node.to_string())
        }

        pub fn get_attr(&self, attr_key: &str) -> Option<&str> {
            let attr_value = self.attrs.iter().find(|&a| a.0.as_str() == attr_key);

            match attr_value {
                None => Some(""),
                Some(a) => Some(a.1.as_str()),
            }
        }
    }
}
