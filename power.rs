fn power(a:i32, b:u32)-> i32 {
    if b==0 {
        return 1
    }
    else {
        return a * power(a,b-1)
    }
}

fn main(){
    println!("{}", power(3,4));
}