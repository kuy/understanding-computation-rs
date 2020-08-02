fn main() {
    let exp = Add {
        left: Multiply {
            left: Number(1),
            right: Number(2),
        },
        right: Multiply {
            left: Number(3),
            right: Number(4),
        },
    };
    println!("{:?}", exp);
}

#[derive(Debug)]
struct Number(i32);

#[derive(Debug)]
struct Add {
    pub left: Multiply,
    pub right: Multiply,
}
#[derive(Debug)]
struct Multiply {
    pub left: Number,
    pub right: Number,
}
