pub fn run() {
    struct Triangle {
        base: u32,
        height: u32
    }

    impl Triangle {
        fn area(&self) -> u32 {
            self.base * self.height / 2
        }
    }

    let my_triangle = Triangle {
        base: 5,
        height: 10
    };

    println!("{}", my_triangle.area());
}
