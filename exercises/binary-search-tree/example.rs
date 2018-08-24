#[derive(Debug, PartialEq)]
pub struct TreeNode {
    element: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

pub struct BinarySearchTree {
    root: Option<Box<TreeNode>>,
}
impl TreeNode {
    pub fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
    pub fn with_right_node(mut self, right_node: TreeNode) -> Self {
        self.right = Some(Box::new(right_node));
        self
    }
    pub fn with_left_node(mut self, left_node: TreeNode) -> Self {
        self.left = Some(Box::new(left_node));
        self
    }
}
impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
        }
    }

    pub fn get_root(&self) -> Option<&TreeNode> {
        self.root.as_ref().map(|b| &**b)
    }

    pub fn get_root_mut(&mut self) -> Option<&mut TreeNode> {
        self.root.as_mut().map(|b| &mut **b)
    }

    pub fn insert(&mut self, element: i32) {
        let node = TreeNode::new(element);
        self.root = Some(Self::insert_node(node, self.root.take()));
    }

    fn insert_node(node: TreeNode, root: Option<Box<TreeNode>> ) -> Box<TreeNode> {
        match root {
            None => Box::new(node),
            Some(mut root) => {
                if node.element <= root.element {
                    root.left = Some(Self::insert_node(node, root.left.take()));
                } else {
                    root.right = Some(Self::insert_node(node, root.right.take()));
                }
                root
            }
        }
    }
}

fn traverse(node: &TreeNode, acc_vec: &mut Vec<i32>) {
    if let Some(left) = node.left.as_ref() {
        traverse(left, acc_vec);
    }
    acc_vec.push(node.element);
    if let Some(right) = node.right.as_ref() {
        traverse(right, acc_vec);
    }
}

impl Into<Vec<i32>> for BinarySearchTree {
    fn into(self) -> Vec<i32> {
        let mut result = vec![];
        if let Some(root) = self.get_root() {
            traverse(root, &mut result);
        }
        result
    }
}
