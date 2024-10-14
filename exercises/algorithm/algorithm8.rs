/*
    queue
    This question requires you to use queues to implement the functionality of the stack
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    // TODO
    mode1: bool,
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            // TODO
            mode1: true,
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        if self.mode1 {
            self.q2.enqueue(elem);
            while let Ok(v) = self.q1.dequeue() {
                self.q2.enqueue(v);
            }
            self.mode1 = false;
        } else {
            self.q1.enqueue(elem);
            while let Ok(v) = self.q2.dequeue() {
                self.q1.enqueue(v);
            }
            self.mode1 = true;
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        if self.is_empty() {
            Err("Stack is empty")
        } else {
            if self.mode1 {
                Ok(self.q1.dequeue().unwrap())
            } else {
                Ok(self.q2.dequeue().unwrap())
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        //TODO
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
