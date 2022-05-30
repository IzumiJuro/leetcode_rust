
struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            stack: vec![],
            min: vec![]
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min.is_empty() || val <= *self.min.last().unwrap() {
            self.min.push(val);
        }
    }

    fn pop(&mut self) {
        if self.stack.is_empty() {
            return ;
        }
        if self.stack.pop().unwrap() == *self.min.last().unwrap() {
            self.min.pop();
        }
    }

    fn top(&self) -> i32 {
        return *self.stack.last().unwrap();
    }

    fn get_min(&self) -> i32 {
        return *self.min.last().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_4() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2)
    }
}

