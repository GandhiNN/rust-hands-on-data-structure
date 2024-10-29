#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };

    if let Some(ref mut v) = ll.next {
        v.add_up(10);
    }

    println!("Hello {:?}", ll);

    let mut v: Vec<String> = vec!["hello".to_string(), "goodbye".to_string()];
    println!("v.len={}, v.capacity={}", v.len(), v.capacity());
    v.push("hohoho".to_string());
    println!("v.len={}, v.capacity={}", v.len(), v.capacity());

    let mut v2: Vec<String> = Vec::with_capacity(100);
    v2.push("yoyo".to_string());
    v2.push("hehe".to_string());
    println!("v2.len={}, v2.capacity={}", v2.len(), v2.capacity());

    for i in 0..105 {
        v2.push(i.to_string());
    }
    println!("v2.len={}, v2.capacity={}", v2.len(), v2.capacity());
}
