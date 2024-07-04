use std::fmt;
#[derive(Debug)] // Derive Debug trait
struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    fn new(value: u32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
    fn left(mut self, node: Node) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    fn right(mut self, node: Node) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    fn set_value(&mut self, new_value: u32) {
        self.value = new_value;
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_tree(f, "", true)
    }
}

impl Node {
    fn display_tree(&self, f: &mut fmt::Formatter<'_>, prefix: &str, is_left: bool) -> fmt::Result {
        if let Some(right) = &self.right {
            right.display_tree(
                f,
                &format!("{}{}", prefix, if is_left { "|   " } else { "    " }),
                false,
            )?;
        }
        write!(
            f,
            "{}{}{}\n",
            prefix,
            if is_left { "|-- " } else { "|-- " },
            self.value
        )?;
        if let Some(left) = &self.left {
            left.display_tree(
                f,
                &format!("{}{}", prefix, if is_left { "    " } else { "|   " }),
                true,
            )?;
        }
        Ok(())
    }
}

use std::collections::VecDeque;

fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut c_table: VecDeque<&Node> = VecDeque::new();
    let mut r_table: Vec<u32> = Vec::new();
    c_table.push_back(root);

    while let Some(node) = c_table.pop_front() {
        r_table.push(node.value);

        if let Some(left_node) = &node.left {
            c_table.push_back(left_node);
        }
        if let Some(right_node) = &node.right {
            c_table.push_back(right_node);
        }
    }

    r_table
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

// Use the builder pattern to create your own tests:
//   let root = Node::new(1)           // create root
//              .left(Node::new(2))    // chain left child (returns root)
//              .right(Node::new(3));  // chain right child (returns root)

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn root_only() {
        assert_eq!(
            tree_by_levels(&Node::new(42)),
            [42],
            "\nYour result (left) didn't match the expected output (right)."
        );
    }

    #[test]
    fn complete_tree() {
        let root = Node::new(1)
            .left(Node::new(2).left(Node::new(4)).right(Node::new(5)))
            .right(Node::new(3).left(Node::new(6)));
        assert_eq!(
            tree_by_levels(&root),
            [1, 2, 3, 4, 5, 6],
            "\nYour result (left) didn't match the expected output (right)."
        );
    }
}
