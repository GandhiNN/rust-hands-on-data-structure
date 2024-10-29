// The stack consists of two data types
// the nodes that own the hierarchy
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// and the stack itself that owns the top node
struct Stack<T> {
    top: Option<Node<T>>,
}

// `next` and `top` can be empty -> marking the end of the stack
// we have to wrap the type into an `Option`
// Because `Node` is a recursive type, we have to wrap its `next`
// value with a `Box`
// This ensures only the pointer gets stored in the data structure
// and hence the size is known to the compiler.
impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node { data, next: None }
    }
}

// A similar thing we can do for the `Stack`
impl<T> Stack<T> {
    // `new` is an object builder
    fn new() -> Stack<T> {
        Stack { top: None }
    }
    // `is_empty` looks at the top node to determine if the stack is empty
    fn is_empty(&self) -> bool {
        self.top.is_none() // return `true` if none, else `false`
    }
    fn push(&mut self, data: T) {
        let mut node = Node::new(data);
        if let Some(top) = self.top.take() {
            // returns the former owned value of self.top and replace it with None
            // Then we match the returned value into the pattern Some(top) and thus
            // gain ownership of the former `top` value
            node.next = Some(Box::new(top));
        }
        self.top = Some(node);
    }
    fn pop(&mut self) -> Option<T> {
        if let Some(top) = self.top.take() {
            self.top = top.next.map(|n| *n);
            Some(top.data)
        } else {
            None
        }
    }
    fn peek(&self) -> Option<&T> {
        match &self.top {
            Some(top) => Some(&top.data),
            None => None,
        }
    }
    fn peek_next(&self) -> Option<&T> {
        match &self.top {
            Some(top) => match &top.next {
                Some(next) => Some(&next.data),
                None => None,
            },
            None => None,
        }
    }
}

fn main() {
    // Test peek
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    println!(
        "Testing peek function => peeked value = {:?} vs test case = {:?}",
        stack.peek(),
        Some(&2)
    );

    // Test peek next
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    println!(
        "Testing peek next function => peeked value next = {:?} vs test case = {:?}",
        stack.peek_next(),
        Some(&1)
    );

    // Test pop
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    println!(
        "Testing first pop => first pop = {:?} vs test case = {:?}",
        stack.pop(),
        Some(2)
    );
    println!(
        "Testing second pop => second pop = {:?} vs test case = {:?}",
        stack.pop(),
        Some(1)
    );

    // Test is empty
    println!(
        "Testing is empty stack => {:?} vs {:?}",
        stack.is_empty(),
        true
    );
}
