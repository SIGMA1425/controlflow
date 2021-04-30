fn main() {
    //if
    let x = 41;
    if x == 41{
        println!("x is 41");
    }
    else if x < -1{
        println!("x is negative");
    }
    else{
        println!("x is positive");
    }

    //if は式なので値を返す
    let x = -41;
    let abs = if x < 0 {-x}else{x};
    println!("abs is {}", abs);

    //for in イテレータで記述する
    for i in 1..5{
        println!("i:{}", i);
    }

    let a = [1,2,3,4];

    for i in a.iter(){
        println!("i:{}", i);
    }
}
