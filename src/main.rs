use serde::{Serialize, Deserialize}; //file save
use std::io::prelude::*; //file ops
use std::fs::File; //files
use std::path::Path; //filepaths
use std::collections::HashMap; //mutable property storage
use crypto::sha1::Sha1;
use crypto::digest::Digest;

#[derive(Serialize, Deserialize, Debug)]
struct SimResult {
    result : HashMap <String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Component {
    name : String,
    typ : i8,
    props : HashMap<String, f64>,
}
impl Component {
    pub fn has_prop (&self, key : &str) -> bool {
        return self.props.contains_key(key);
    }
    pub fn prop (&self, key : &str) -> f64 {
        return self.props[key];
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Rocket {
    components : Vec<Component>,
    precompute : HashMap<String, f64>,
    prop_hash : String,
}
impl Rocket {
    pub fn new() -> Self {
        Self{
            components : Vec::new(),
            precompute : HashMap::new(),
            prop_hash : String::new(),
        }
    }
    fn hash (&mut self) {
        self.precompute_mprops();
        let mut data : f64 =  0.0;
        for prop in &self.precompute {
            data += prop.1;
        }
        let mut hasher = Sha1::new();
        hasher.input_str(format!("{}",data).as_str());
        self.prop_hash = hasher.result_str();
    }
    pub fn add_component(&mut self, c : Component){
        self.components.push(c)
    }
    pub fn write_out (&mut self, p : String) {
        self.hash();
        let data = serde_json::to_string_pretty(&self).unwrap();
        let to = Path::new(&p);
        let display = to.display();

        let mut out_file = match File::create(to) {
            Err(why) => panic!("couldn't create to {}: {}", display, why),
            Ok(out_file) => out_file, 
        };
        match out_file.write_all(data.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
    //summary functions
    fn max_diameter (&self) -> f64 {
        let mut max_dia : f64 = 0.0; 
        for comp in &self.components {
            if comp.has_prop("diameter") {
                if comp.props["diameter"] > max_dia {
                    max_dia = comp.props["diameter"];
                }
            }
        }
        return max_dia;
    }

    fn total_mass (&self) -> f64 {
        let mut mass_acc : f64 = 0.0;
        for comp in &self.components {
            if comp.has_prop("mass") {
                mass_acc += comp.props["mass"];
            }
        }
        return mass_acc;
    }

    fn total_length (&self) -> f64 {
        let mut length_acc : f64 = 0.0;
        for comp in &self.components {
            if comp.has_prop("length") {
                length_acc += comp.prop("length");
            }
        }
        return length_acc; 
    }

    fn center_of_mass (&self) -> f64 {
        let mut sum_pos : f64 = 0.0;
        for comp in &self.components {
            if comp.has_prop("position"){
                sum_pos += comp.prop("position");
            }
        }
        return (self.total_mass() * sum_pos) / sum_pos;
    }
    pub fn precompute_mprops (&mut self) {
        self.precompute.insert(String::from("mass"), self.total_mass());
        self.precompute.insert(String::from("com"), self.center_of_mass());
        self.precompute.insert(String::from("dia"), self.max_diameter());
        self.precompute.insert(String::from("length"), self.total_length());
    }
    pub fn display (&mut self,  mut indent : usize) {
        indent = if indent != 4 {indent} else {4};  
        for comp in &self.components {
            println!("Name {} \n Prop. name; Prop. value", comp.name);
            for p in &comp.props {
                println!("{tab}{};{}", p.0, p.1, tab=" ".repeat(indent))
            }
        }
        println!("--------------");
        self.precompute_mprops();
        println!("Major Properties \n Name; Value");
        for p in &self.precompute {
            println!("{tab}{};{}", p.0, p.1, tab=" ".repeat(indent))
        }
        println!("--------------");
        self.hash();
        println!("prophash:{}",self.prop_hash)
    }
}

fn main() {
    println!("testing");
    let mut R = Rocket::new();

    let comp1 = Component {
        name : String::from("comp1"),
        typ : 0,
        props : HashMap::from([
            (String::from("mass"), 3.14),
            (String::from("diameter"), 5.63),
            (String::from("position"), 12.3),
            (String::from("length"), 242.5),
        ])
    };
    R.add_component(comp1);

    let comp2 = Component {
        name : String::from("comp2"),
        typ : 1 ,
        props : HashMap::from([
            (String::from("mass"), 2.56),
            (String::from("diameter"), 44.5),
            (String::from("position"), 3.4565),
            (String::from("length"), 4.565),
        ]) 
    };
    R.add_component(comp2);
    R.display(4);
    R.write_out(String::from("./testing/test.json"));
}
