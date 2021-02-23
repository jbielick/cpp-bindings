use super::{Field, FieldType};

pub(crate) struct NodeCpp<'a> {
    nodes: &'a [lib_ruby_parser_nodes::Node],
}

impl<'a> NodeCpp<'a> {
    pub(crate) fn new(nodes: &'a [lib_ruby_parser_nodes::Node]) -> Self {
        Self { nodes }
    }

    pub(crate) fn write(&self) {
        std::fs::write("../src/node.cpp", self.contents()).unwrap()
    }

    fn contents(&self) -> String {
        format!(
            "#include \"node.h\"

namespace lib_ruby_parser {{

{implementations}

}}
",
            implementations = self.implementations().join("\n")
        )
    }

    fn implementations(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|n| NodeImplementation::new(n).code())
            .collect()
    }
}

struct NodeImplementation<'a> {
    node: &'a lib_ruby_parser_nodes::Node,
}

impl<'a> NodeImplementation<'a> {
    fn new(node: &'a lib_ruby_parser_nodes::Node) -> Self {
        Self { node }
    }

    fn code(&self) -> String {
        format!(
            "{class_name}::{class_name}({args})
{{
{constructor}
}}
",
            class_name = self.node.struct_name,
            args = self.args().join(", "),
            constructor = self.construtor_stmts().join("\n")
        )
    }

    fn args(&self) -> Vec<String> {
        self.node
            .fields
            .iter()
            .map(|f| {
                format!(
                    "{field_type} {field_name}",
                    field_type = FieldType::new(&f.field_type).cpp_type(),
                    field_name = Field::new(f).cpp_name()
                )
            })
            .collect()
    }

    fn construtor_stmts(&self) -> Vec<String> {
        self.node
            .fields
            .iter()
            .map(|f| {
                let field_name = Field::new(f).cpp_name();

                let mut rhs = field_name.clone();
                if FieldType::new(&f.field_type).needs_move() {
                    rhs = format!("std::move({})", rhs)
                }

                format!(
                    "this->{field_name} = {rhs};",
                    field_name = field_name,
                    rhs = rhs
                )
            })
            .collect()
    }
}
