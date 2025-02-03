//passing by VALUE
// fn main(){
//     let arr: [&str;3]=["aryan","kavish","mehul"];//use semicolon while defining
//     writearr(arr);
//     println!("{:?}",arr);
// }
// fn writearr(mut arr1:[&str;3]){
//     arr1[0]="gurkiran";
//     println!("{:?}",arr1);
    
// 
//now this is how we pass through a reference
fn main(){
    let mut arr: [&str;3]=["aryan","kavish","mehul"];//use semicolon while defining
    writearr(&mut arr);//remember to use &mut
    println!("{:?}",arr);
}
fn writearr(arr1:&mut [&str;3]){
    arr1[0]="gurkiran";
    println!("{:?}",arr1);
    
}