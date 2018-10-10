pub mod graph {
    use std::collections::HashMap;

    #[derive(Clone)]
    pub struct Graph {
        pub edges: Vec<graph_items::edge::Edge>,
        pub nodes: Vec<graph_items::node::Node>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                edges: Vec::new(),
                nodes: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            self.nodes = Vec::from(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
            self.edges = Vec::from(edges);
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            self
        }

        pub fn get_node<'a>(&'a self, nodename: &str) -> Option<&'a graph_items::node::Node> {
            for n in self.nodes.iter() {
                if n.name == nodename {
                    return Some(n);
                }
            }
            return None;
        }

        pub fn get_attr<'a>(&'a self, attr: &str) -> Option<&'a String> {
            self.attrs.get(attr)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                pub first: String,
                pub second: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(first: &str, second: &str) -> Self {
                    Edge {
                        first: first.to_string(),
                        second: second.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }

                pub fn get_attr<'a>(&'a self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| s.as_str())
                }
            }

        }
    }
}
