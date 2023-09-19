fn main() {
    println!("Hello, world!");
    let mut x = 6;
    let mut result: i64;
    let mut best_result = 0;
    let mut best_num = 1;
    while true {
        result = congettura(x);
        if result > best_result {best_result = result; best_num = x; }
        println!("br: {} with num {}... {} -> {} steps before getting to 1",best_result, best_num, x, result);
        x = x+1;
    }
}

fn congettura(mut n: i64) -> i64{
    let mut cont = 0;
    while n>1 {
        if n%2!=0 {
            n = 3*n+1;
        }else{
            n = n/2;
        }
        cont += 1;
    }
    cont
}
