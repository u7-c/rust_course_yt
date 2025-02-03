//we know it is heap allocated now we will see ownership rules more often
//its like heap allocation and dynamic memory
fn main(){
    let mut vrr:Vec<&str>=vec!["aryan","mehul","kavish"];
    write_vrr(&mut vrr);
    println!("the updated vector is {:?}",vrr);
}
fn write_vrr(vrr2: &mut Vec<&str>){
    vrr2.push("gurkiran");
    
}
//u forgot to write syntax 
//u wrote the argument while the function wrong
//Vec with a V capital
