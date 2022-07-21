use std::{ops::Deref, rc::Rc};

pub trait FunctionalStackOps<T>: Sized {
    fn top(&self) -> Option<&T>;
    fn pop(&self) -> Option<Self>;
    fn push(&self, el: T) -> Self;
    fn empty(&self) -> bool;
}

pub enum FunctionalStack<T> {
    Cons(T, StackPointer<T>),
    Nil,
}

pub struct StackPointer<T>(Rc<FunctionalStack<T>>);
impl<T> StackPointer<T> {
    pub fn new() -> Self {
        Self(Rc::new(FunctionalStack::Nil::<T>))
    }
}
impl<T> From<FunctionalStack<T>> for StackPointer<T> {
    fn from(stack: FunctionalStack<T>) -> Self {
        Self(Rc::new(stack))
    }
}
impl<T> From<Vec<T>> for StackPointer<T> {
    fn from(list: Vec<T>) -> Self {
        let mut stack = Self::new();
        for item in list {
            stack = Self::from(FunctionalStack::Cons(item, stack));
        }
        stack
    }
}
impl<T> Clone for StackPointer<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
impl<T> Deref for StackPointer<T> {
    type Target = FunctionalStack<T>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl<T> FunctionalStackOps<T> for StackPointer<T> {
    fn empty(&self) -> bool {
        match **self {
            FunctionalStack::Cons(_, _) => false,
            FunctionalStack::Nil => true,
        }
    }
    fn pop(&self) -> Option<Self> {
        match self.deref() {
            FunctionalStack::Cons(_, stack) => Some(stack.clone()),
            FunctionalStack::Nil => None,
        }
    }
    fn push(&self, el: T) -> Self {
        Self::from(FunctionalStack::Cons(el, self.clone()))
    }
    fn top(&self) -> Option<&T> {
        match self.deref() {
            FunctionalStack::Cons(el, _) => Some(el),
            FunctionalStack::Nil => None,
        }
    }
}
impl<T: Clone> Iterator for StackPointer<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.top()?.clone();
        match self.pop() {
            Some(stack) => self.0 = Rc::clone(&stack.0),
            None => self.0 = Rc::new(FunctionalStack::Nil),
        };
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use crate::{FunctionalStackOps, StackPointer};

    #[test]
    fn should_be_empty() {
        let stack = StackPointer::<usize>::new();
        assert!(stack.empty());
    }
    #[test]
    fn should_push_immutably() {
        let stack1 = StackPointer::<usize>::new();
        let stack2 = stack1.push(2);
        let stack3 = stack1.push(3);
        let stack4 = stack2.push(4);
        assert!(stack1.into_iter().eq([].into_iter()));
        assert!(stack2.into_iter().eq([2].into_iter()));
        assert!(stack3.into_iter().eq([3].into_iter()));
        assert!(stack4.into_iter().eq([4, 2].into_iter()));
    }
    #[test]
    fn top_works() {
        let stack = StackPointer::<usize>::new();
        assert_eq!(stack.top(), None);
        let stack = stack.push(0);
        assert_eq!(stack.top(), Some(&0));
        let stack = stack.push(1);
        assert_eq!(stack.top(), Some(&1));
        let stack = stack.push(2);
        assert_eq!(stack.top(), Some(&2));
        let stack = stack.push(3);
        assert_eq!(stack.top(), Some(&3));
    }
    #[test]
    fn pop_is_immutable() {
        let stack = StackPointer::from(vec![1, 2, 3, 4, 5]);
        let stack2 = stack.pop().unwrap();
        let stack3 = stack.pop().unwrap();
        let stack4 = stack2.pop().unwrap();
        let stack5 = stack4.pop().unwrap();
        assert!(stack.into_iter().eq([1, 2, 3, 4, 5].into_iter().rev()));
        assert!(stack2.into_iter().eq([1, 2, 3, 4].into_iter().rev()));
        assert!(stack3.into_iter().eq([1, 2, 3, 4].into_iter().rev()));
        assert!(stack4.into_iter().eq([1, 2, 3].into_iter().rev()));
        assert!(stack5.into_iter().eq([1, 2].into_iter().rev()));
    }
}
