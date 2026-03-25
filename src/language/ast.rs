use crate::language::nodes::Node;

pub struct AST {
    pub nodes: Vec<Node>,
}

impl AST {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self { nodes }
    }

    pub fn is_terminator(node: &Node) -> bool {
        if let Node::BreakStatement(_) = node {
            true
        } else if let Node::ReturnStatement(_) = node {
            true
        } else if let Node::OutStatement(_) = node {
            true
        } else if let Node::ContinueStatement = node {
            true
        } else {
            false
        }
    }

    pub fn prune_ast(&mut self) {
        for node in self.nodes.iter_mut() {
            Self::prune_node(node)
        }
    }

    pub fn prune_node(node: &mut Node) {
        match node {
            Node::ExprStmt(node) => {
                Self::prune_node(node);
            }
            Node::Block { body } => Self::prune_block(body),
            Node::SingleLineBlock { body } => Self::prune_node(body),
            Node::IfStatement {
                block,
                elifs,
                else_block,
                condition,
            } => {
                if let Node::BooleanLiteral(x) = **condition {
                    if x == true {
                        Self::prune_node(block);
                        *node = *block.clone();
                        return;
                    }
                }

                Self::prune_node(condition);
                for (a, b) in elifs {
                    Self::prune_node(a);
                    Self::prune_node(b);
                }
                if let Some(e_block) = else_block {
                    Self::prune_node(e_block)
                }
            }
            Node::LetStatement { values, .. } => {
                for val in values {
                    if let Some(v) = val {
                        Self::prune_node(v);
                    }
                }
            }
            Node::ReturnStatement(value) => {
                if let Some(v) = value {
                    Self::prune_node(v);
                }
            }
            Node::OutStatement(value) => {
                if let Some(v) = value {
                    Self::prune_node(v);
                }
            }
            Node::BreakStatement(value) => {
                if let Some(v) = value {
                    Self::prune_node(v);
                }
            }
            Node::FunctionDefinition { block, .. } => {
                Self::prune_node(block);
            }
            Node::Loop { block, .. } => {
                Self::prune_node(block);
            }
            Node::ForLoop { block, expr, .. } => {
                Self::prune_node(expr);
                Self::prune_node(block);
            }
            Node::WhileLoop {
                block, condition, ..
            } => {
                Self::prune_node(condition);
                Self::prune_node(block);
            }
            Node::BinOp { left, right, .. } => {
                Self::prune_node(left);
                Self::prune_node(right);
            }
            Node::UnaryOp { right, .. } => {
                Self::prune_node(right);
            }
            Node::DictNode(values) => {
                for (a, b) in values {
                    Self::prune_node(a);
                    Self::prune_node(b);
                }
            }
            Node::ListNode(values) => {
                for val in values {
                    Self::prune_node(val);
                }
            }
            Node::TupleNode(values) => {
                for val in values {
                    Self::prune_node(val);
                }
            }
            Node::RangeNode {
                start, step, end, ..
            } => {
                Self::prune_node(start);
                Self::prune_node(end);
                if let Some(v) = step {
                    Self::prune_node(v);
                }
            }
            Node::FunctionCall { target, args } => {
                Self::prune_node(target);

                for val in args {
                    Self::prune_node(val);
                }
            }
            Node::MemberAccess { expr, member } => {
                Self::prune_node(expr);
                Self::prune_node(member);
            }
            Node::SetVariable { target, value } => {
                Self::prune_node(target);
                Self::prune_node(value);
            }
            Node::ShorthandAssignment { target, value, .. } => {
                Self::prune_node(target);
                Self::prune_node(value);
            }

            _ => {}
        }
    }

    pub fn prune_block(body: &mut Vec<Node>) {
        for (idx, i) in body.iter_mut().enumerate() {
            Self::prune_node(i);
            if let Node::ExprStmt(i) = i {
                if Self::is_terminator(&*i) {
                    body.truncate(idx + 1);
                    return;
                }
            } else if Self::is_terminator(&*i) {
                body.truncate(idx + 1);
                return;
            }
        }
    }
}
