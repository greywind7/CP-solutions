// https://www.hackerrank.com/challenges/bon-appetit/problem

macro_rules! par{
    ($a:expr,$b:ty) => {
        $a.parse::<$b>().unwrap()
    }
}

fn inp(n:i32) -> (i32,usize) {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp);
    let mut it = inp.split_whitespace();
    let a = it.next().unwrap().parse::<i32>().unwrap();
    if n == 1{
        return (a,0);
    }
    let b = it.next().unwrap().parse::<usize>().unwrap();
    return (a,b);
} 

fn inp_vec(v:&mut Vec<i32>){
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp);
    let mut it = inp.split_whitespace();
    for i in it {
        v.push(par!(i,i32))
    }
}

fn main(){
    let (n,k) = inp(2);
    let mut arr = Vec::new();
    inp_vec(&mut arr);
    let calc = inp(1).0;
    let mut fair = 0;
    for t in arr.iter(){
        fair+=*t;
    }
    fair -= arr[k];
    fair /= 2;
    if calc == fair{
        println!("Bon Appetit");    
    }
    else{
        println!("{}",calc-fair);
    }
    
}