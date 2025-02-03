//vector initialising
fn main(){
    let mut v=vec![1,2,3,4];
    v.push(10);
    println!("{:?}",v);
    v.pop();
    println!("{:?}",v);
}