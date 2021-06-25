// https://www.hackerrank.com/challenges/luck-balance/problem
use std::collections::BinaryHeap;

fn inp()->(i32,i32){
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp);
    let mut it = inp.split_whitespace();
    let a = it.next().unwrap().parse::<i32>().unwrap();
    let b = it.next().unwrap().parse::<i32>().unwrap();
    (a,b)
}

fn main()
{
    let mut vals = BinaryHeap::new();
    let (n,k) = inp();
    let mut tot = 0;
    for _ in 0..n {
        let(l,p) = inp();
        if p == 1{
            vals.push(l);
        }
        else{
            tot += l;
        }
    }
    let mut tmp = 0;
    
    if k > 0{
        while let Some(val) = vals.pop(){
            tot+= val;
            tmp+=1;
            if tmp == k{
                break;
        }
    }
}
    
    while let Some(val) = vals.pop(){
        tot-=val;
    }
    print!("{}",tot);
}