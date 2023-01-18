fn swap<P1, P2>(a:P1, b:P2) -> (P2,P1){
    (b,a)
}

// #![cfg(test)]
// #[test]

fn main(){
    
    assert_eq!((5,10), swap(10,5));
    assert_eq!((5,true), swap(true,5));
    assert_eq!(('a',false), swap(false,'a')); 
    
    println!("Success");

}