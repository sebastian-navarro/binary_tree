#[warn(dead_code)]
pub struct Node <T> {
    value : T, // T value because could be string , integer or other values
    left : Option<Box<Node<T>>>, // Option because could be None or Value
    rigth: Option<Box<Node<T>>>,
}

impl <T> Node<T> {
    pub fn new(value : T) -> Self {
        Node {
            value,
            left: None,
            rigth: None
        }   
    }
    pub fn left(mut self, node: Node<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn rigth(mut self, node: Node<T>) -> Self {
        self.rigth = Some(Box::new(node));
        self
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_tree() {
        let tree = Node::new(1); 
        assert_eq!(tree.value, 1); 
    }
}
