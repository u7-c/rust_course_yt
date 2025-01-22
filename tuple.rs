fn main(){
    let empinfo:(&str,u8)=("ramesh",50);
    let empname=empinfo.0;
    let empage=empinfo.1;
    println!("the name is {}",empname);
    
    //---this is the normal way below we will se how to destructure t as we do itin javascript
    let (empname,empage)=empinfo;
    println!("the name is {}",empname);
    println!("the age is {}",empage);
    //there are two ways this is just like we use structure in c tod efine multiple members at once
}