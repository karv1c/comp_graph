use std::{cell::RefCell, rc::Rc};

enum Task {
    Add,
    Mul,
    Sin,
    Pow,
    None,
}

pub struct Node {
    _label: String,
    value: RefCell<Option<f32>>,
    task: Task,
    nodes: RefCell<Option<(Rc<Node>, Rc<Node>)>>,
    cache: RefCell<Option<Vec<f32>>>,
}
impl Node {
    pub fn set(&self, value: f32) {
        *self.value.borrow_mut() = Some(value)
    }
    pub fn compute(&self) -> f32 {
        let mut args: Vec<f32> = Vec::new();
        let mut result;
        match self.nodes.borrow().as_ref() {
            Some(nodes) => {
                args.push(nodes.0.compute());
                args.push(nodes.1.compute());
                match self.cache.clone().borrow().as_ref() {
                    Some(cache) => {
                        if cache == &args {
                            if let Some(value) = self.value.borrow().as_ref() {
                                return *value;
                            }
                        }
                    }
                    None => *self.cache.borrow_mut() = Some(args.clone()),
                }
            }
            None => {
                if let Some(value) = self.value.borrow().as_ref() {
                    return *value;
                } else {
                    panic!("No Value and Nodes in Node!")
                }
                
            }
        }

        match self.task {
            Task::Add => {
                result = 0.0;
                for arg in args {
                    result += arg;
                }
            }
            Task::Mul => {
                result = 1.0;
                for arg in args {
                    result *= arg;
                }
            }
            Task::Sin => {
                result = args[0].sin();
            }
            Task::Pow => result = args[0].powf(args[1]),
            Task::None => {
                if let Some(value) = self.value.borrow().as_ref() {
                    return *value;
                } else {
                    panic!("Node is not initialized correctly!")
                }
            },
        }
        result
    }
}
pub fn create_input(label: &str) -> Rc<Node> {
    Rc::new(Node {
        _label: label.to_owned(),
        value: RefCell::new(None),
        task: Task::None,
        nodes: RefCell::new(None),
        cache: RefCell::new(None),
    })
}

pub fn add(x1: Rc<Node>, x2: Rc<Node>) -> Rc<Node> {
    Rc::new(Node {
        _label: "add".to_string(),
        value: RefCell::new(None),
        task: Task::Add,
        nodes: RefCell::new(Some((x1, x2))),
        cache: RefCell::new(None),
    })
}

pub fn mul(x1: Rc<Node>, x2: Rc<Node>) -> Rc<Node> {
    Rc::new(Node {
        _label: "mul".to_string(),
        value: RefCell::new(None),
        task: Task::Mul,
        nodes: RefCell::new(Some((x1, x2))),
        cache: RefCell::new(None),
    })
}
pub fn sin(x1: Rc<Node>) -> Rc<Node> {
    let arg2_node = Rc::new(Node {
        _label: "sin_dummy".to_string(),
        value: RefCell::new(Some(0.0)),
        task: Task::None,
        nodes: RefCell::new(None),
        cache: RefCell::new(None),
    });
    Rc::new(Node {
        _label: "sin".to_string(),
        value: RefCell::new(None),
        task: Task::Sin,
        nodes: RefCell::new(Some((x1, arg2_node))),
        cache: RefCell::new(None),
    })
}
pub fn pow_f32(x1: Rc<Node>, x2: f32) -> Rc<Node> {
    let x2_node = Rc::new(Node {
        _label: "pow_dummy".to_string(),
        value: RefCell::new(Some(x2)),
        task: Task::None,
        nodes: RefCell::new(None),
        cache: RefCell::new(None),
    });
    Rc::new(Node {
        _label: "pow".to_string(),
        value: RefCell::new(None),
        task: Task::Pow,
        nodes: RefCell::new(Some((x1, x2_node))),
        cache: RefCell::new(None),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute() {
        let x1 = create_input("x1");
        let x2 = create_input("x1");
        let graph = add(x1.clone(), x2.clone());
        x1.set(2.0);
        x2.set(2.0);
        assert_eq!(graph.compute(), 4.0);
    }
    #[test]
    #[should_panic]
    fn test_data() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let graph = add(x1.clone(), x2.clone());
        x1.set(1f32);
        graph.compute();
    }
}