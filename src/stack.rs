use crate::Stack;

// TODO Complete implementation
impl Stack for Vec<i32> {
    fn init() -> Self {
        let vec: Vec<i32> = Vec::new();
        vec
        //todo!()
    }

    fn push_val(&mut self, i: i32) {
        self.push(i);
        //todo!()
    }

    fn top_val(&self) -> Option<&i32> {
        let last: Option<&i32> =self.last();
        match last {
            None => None,
            Some(i) => Some(i),
        }
        //todo!()
    }

    fn pop_val(&mut self) -> Option<i32> {
        let last: Option<i32> = self.pop();
        match last {
            None => None,
            Some(i) => Some(i),
        }
        //todo!()
    }

    fn is_empty(&self) -> bool {
        let is_empty: bool = self.is_empty();
        is_empty
        //todo!()
    }
}

#[derive(Debug)]
pub enum ListStack {
    Val(i32, Option<Box<ListStack>>),
    Nil,
}

use ListStack::Nil;
use ListStack::Val;

// Complete implementation of Stack for ListStack
impl Stack for ListStack {
    fn init() -> Self {
        Nil
    }

    fn push_val(&mut self, i: i32) {
        match self {
            Val(value, other) => {
                *self = Val(i, other.take());
                Nil
            },

            Nil => {
                *self = Nil;
                Nil
            }
        };
    }

    fn top_val(&self) -> Option<&i32> {
        match self {
            Val(value, other) => value,
            Nil => Nil,
        }
        //todo!()
    }

    fn pop_val(&mut self) -> Option<i32> {
        match self {
            Val(value, other) => {
                let popped_value = *value;
                match other.take() {
                    None => *self = Nil,
                    Some(other) => todo!(),
                };
                todo!()
            }
            Nil => None,
        }
    }

    fn is_empty(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::ListStack;
    use crate::Stack;
    use std::fmt::Debug;

    #[test]
    fn vec_fill_and_clear() {
        println! {"Testing Vec<T>"}
        fill_and_clear_impl(Vec::init());
    }

    #[test]
    fn linked_fill_and_clear() {
        println! {"Testing ListStack"}
        fill_and_clear_impl(ListStack::init());
    }

    fn fill_and_clear_impl<T: Stack + Debug>(mut stack: T) {
        stack.push_val(1);
        assert_eq!(stack.top_val(), Some(&1));

        stack.push_val(2);
        assert_eq!(stack.top_val(), Some(&2));

        stack.push_val(-3);
        assert_eq!(stack.top_val(), Some(&-3));

        println!("{:?}", stack);

        let mut comparison = vec![1, 2, -3];
        while let Some(val) = stack.pop_val() {
            assert_eq!(comparison.pop().unwrap(), val);
        }

        assert!(stack.is_empty())
    }
}
