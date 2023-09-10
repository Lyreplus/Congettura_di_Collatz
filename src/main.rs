fn main() {
    println!("Hello, world!");
    let mut x = 6;
    let mut result: i32;
    while true {
        println!("{}", x);
        result = congettura(x);
        println!("{}", result);
        x = x+1;
    }
}

fn congettura(mut n: i32) -> i32{
    while n>1 {
        if n%2!=0 {
            n = 3*n+1;
        }else{
            n = n/2;
        }
        //println!("{}", n);
    }
    n
}
