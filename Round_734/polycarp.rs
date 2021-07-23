fn inp() -> i32{
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp);
    let a = inp.trim().parse::<i32>().unwrap();
    return a
}

fn main(){
    let t = inp();
    for _ in 0..t {
        let mon = inp();
        let tmp = mon/3;
        let (mut a, mut b) = (tmp,tmp);
        if mon % 3 == 1{a+=1}
        else if mon % 3 == 2{b+=1}
        println!("{} {}",a,b)
    }
}