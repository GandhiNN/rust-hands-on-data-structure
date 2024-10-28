pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    let mut st = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };

    for val in &mut st {
        println!("loop {}", val);
    }

    let mut st2 = Stepper {
        curr: 3,
        step: 4,
        max: 15,
    };

    while let Some(v) = st2.next() {
        println!("Hello, {}!", v);
    }

    let it = Stepper {
        curr: 5,
        step: 8,
        max: 100,
    };
    for i in it {
        println!("Stepping: {}", i);
    }

    // let mut n = 0;

    // while n < 10 {
    //     n += 1;
    //     println!("Hello, {}!", n);
    // }

    // for i in 0..10 {
    //     println!("for loop {}", i);
    // }
    println!("All done");
}
