#[derive(Debug)]
struct Node {
    val: char,
    original_pos: usize,
    cost: usize,
    left_child_idx: Option<usize>,
    right_child_idx: Option<usize>,
    parent_idx: Option<usize>
}

#[derive(Debug)]
pub struct Tree {
    chain: Vec<Node>
}

impl Node {
    fn new(val: char, orig: usize, parent: Option<usize>) -> Self {
        Self {
            val,
            original_pos: orig,
            cost: 0,
            left_child_idx: None,
            right_child_idx: None, 
            parent_idx: parent
        }
    }




}

impl Tree {
    pub fn new() -> Self {
        Self {
            chain: vec![]
        }
    }

    pub fn push(&mut self, val: char, pos: usize) {
        self.insert(0, val, pos);
    }

    pub fn update_cost(&mut self) {
        self.chain.iter().for_each(|node| {
            let node_val = node.val;
            let node_left = node.left_child_idx;
        })
    }

    fn get_count(&self, at_idx: usize, cmp: char) -> usize {
        let mut count = 0;
        if let Some(node) = self.chain.get(at_idx) {
            // if node.val <= cmp {
            //     count += 1;
            // }
            
        }
        0
    }




    fn len(&self) -> usize {
        self.chain.len()
    }

    fn add_to_chain(&mut self, node: Node) -> usize {
        self.chain.push(node);
        self.len() - 1
    }
    fn update_left_idx(&mut self, at_idx: usize, child_idx: usize) {
        if let Some(node) = self.chain.get_mut(at_idx) {
            node.left_child_idx = Some(child_idx);
        }
    }
    fn update_right_idx(&mut self, at_idx: usize, child_idx: usize) {
        if let Some(node) = self.chain.get_mut(at_idx) {
            node.right_child_idx = Some(child_idx);
        }
    }

    fn insert(&mut self, at_idx: usize, val: char, pos: usize) {
        if let Some(node) = self.chain.get(at_idx) {
            if val < node.val {
                if let Some(left) = node.left_child_idx {
                    self.insert(left, val, pos);
                } else {
                    let child_node = Node::new(val, pos, Some(at_idx));
                    let child_idx = self.add_to_chain(child_node);
                    self.update_left_idx(at_idx, child_idx);
                }
                return;
            } 

            if let Some(right) = node.right_child_idx {
                self.insert(right, val, pos);
            } else {
                let child_node = Node::new(val, pos, Some(at_idx));
                let child_idx = self.add_to_chain(child_node);
                self.update_right_idx(at_idx, child_idx);
            }
            return;
        }

        let node = Node::new(val, pos, None);
        self.chain.push(node);
    }
}






