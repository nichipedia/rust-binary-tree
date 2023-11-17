use std::{collections::HashMap, cmp};


struct BinaryTree<'a> {
    name: String,
    layers: HashMap<usize, Vec<Node<'a>>>
}

impl BinaryTree<'_> {

    fn cultivate(input: String) -> Self {
        // Read tree from String to build...
        let mut map: HashMap<usize, Vec<Node>> = HashMap::new();
        let rows: Vec<&str> = input.split("\n").collect();
        let root_val: usize = rows[0].parse().unwrap();
        let root = Node::grow(root_val, 0, root_val);
        map.insert(0, vec![root]);
        for i in 1..rows.len()-1 {
            let row: Vec<usize> = rows[i].split(" ").map(|val| val.parse().unwrap()).collect();
            let mut cur_nodes:Vec<Node> = Vec::new();
            for j in 0..row.len() {
                if j == 0 {
                    let &mut parent: &mut Node = map.get_mut(&(i-1)).unwrap().get_mut(0).unwrap();
                    let new_node = Node::grow(row[j].clone(), i, parent.get_sum_path());
                    parent.graft_left(&new_node);
                    cur_nodes.push(new_node);
                } else if j > 0 && j < row.len() - 1 {
                    let &mut left_parent: &mut Node = map.get_mut(&(i-1)).unwrap().get_mut(j-1).unwrap();
                    let &mut right_parent: &mut Node = map.get_mut(&(i-1)).unwrap().get_mut(j).unwrap();
                    let left_sum = left_parent.get_sum_path();
                    let right_sum = right_parent.get_sum_path();
                    let node_val = row[j].clone();
                    let new_node = Node::grow(node_val, i, cmp::max(left_sum, right_sum) + node_val);
                    left_parent.graft_right(&new_node);
                    right_parent.graft_left(&new_node);
                    cur_nodes.push(new_node);
                } else {
                    let &mut parent: &mut Node = map.get_mut(&(i-1)).unwrap().get_mut(j-2).unwrap();
                    let new_node = Node::grow(row[j].clone(), i, parent.get_sum_path());
                    parent.graft_right(&new_node);
                    cur_nodes.push(new_node);
                }
            }
            map.insert(i, cur_nodes);
        }

        return BinaryTree { name: String::from("Binary Tree Implementation"), layers: map };
    }

    fn get_root(&self) -> &Node {
        return self.layers.get(&0).unwrap().get(0).unwrap();
    }

}

struct Node<'a> {
    level: usize,
    val: usize,
    right_branch: Option<Box<&'a Node<'a>>>,
    left_branch: Option<Box<&'a Node<'a>>>,
    sum_path: usize,
}

impl<'a> Node<'a> {
    fn grow(val: usize, level: usize, sum_path: usize) -> Self {
        Self {
            level,
            val,
            sum_path,
            right_branch: None,
            left_branch: None,
        }
    }

    fn get_sum_path(&self) -> usize {
        return self.sum_path.clone();
    }

    fn graft_right(&mut self, node: &'a Node) {
        self.right_branch = Option::Some(Box::new(node));
    }

    fn graft_left(&mut self, node: &'a Node) {
        self.left_branch = Option::Some(Box::new(node));
    }
}


fn main() {
    println!("Hello, world!");
}
