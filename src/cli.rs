use crate::ModelRocket as MR;
use strum::IntoEnumIterator;
use std::string::String;
pub fn read() -> String {
    let mut inp = String::new();
    std::io::stdin()
            .read_line(&mut inp)
            .expect("Read Failiure, exiting");
    return inp;
}
pub fn build_component(componentType : MR::ComponentTypeId) {
    if componentType == MR::ComponentTypeId::Nosecone {
        //nosecone params
    }
    if componentType == MR::ComponentTypeId::Bodytube {

    }
    if componentType == MR::ComponentTypeId::InnerTube {

    }
    if componentType == MR::ComponentTypeId::Fins{

    }
}
/// Get f64 parameter from terminal
fn promptf64(p: &str) -> f64 {
    println!("{}",p);
    let mut inp = String::new();
    //let mut out: f64;
    let mut out : f64 = 0.0; 
    std::io::stdin()
            .read_line(&mut inp)
            .expect("Read Failiure, exiting");
    out = match inp.parse::<f64>() {
        Ok(_) => out, 
        _ => 0.0, //* this should probaly use a Result<> idk
    };
    return out;
}
fn prompt_ComponentTypeId(p: &str) -> MR::ComponentTypeId {
    println!("{}",p);
    let mut inp = String::new();
    //let mut out: f64;
    let mut out = MR::ComponentTypeId::Null; 
    std::io::stdin()
            .read_line(&mut inp)
            .expect("Read Failiure, exiting");
    out = match inp.parse::<f64>() {
        Ok(_) => out, 
        _ => MR::ComponentTypeId::Null, //* this should probaly use a Result<> idk
    };
    println!("{}",out);
    return out;
}
pub fn get_component_type() -> MR::ComponentTypeId {
    let mut typcount : i8 = 0;
    for typ in MR::ComponentTypeId::iter() {
        println!("{}:{:?}",typcount,typ);
        typcount+=1;
    }
    let choice = prompt_ComponentTypeId("Which of the following components would you like to insert?");
    return choice;
}
pub fn dumpfile(pth : &str){
    println!("{}",std::fs::read_to_string(pth).expect("Check inputted path, it is incorrect or unavailible"));
}
