fn all_fib(n:u32){
    for num in 1..n {
        println!("{}: {}", num, fib(num));
    }
}

fn fib(n:u32) -> u32 {
    if n==0 {return 0}
    else if n==1 {return 1}
    return fib(n-1) + fib(n-2)
}

fn main(){
    all_fib(15);
}