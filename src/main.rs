//! rs_ModelRocket: A command-line model rocketry simulator


use ctrlc;

/// This module contains all things having to do with the actual simulation,
/// including the algorihims and their corresponding data transfer layers.
mod Sim{
    use serde::{Serialize,Deserialize};
    use std::collections::HashMap;
    #[derive(Serialize, Deserialize)]
    struct SimResult {
        result : HashMap <String, f64>,
    }
}

mod ModelRocket{
    use serde::{Serialize,Deserialize};
    use std::collections::HashMap;
    use std::fmt::{self,Display,Formatter};
    use crypto::sha1::Sha1;
    use crypto::digest::Digest;
    use std::io::prelude::*; //file ops
    use std::fs::File; //files
    use std::path::Path; //filepaths
    use std::process::exit;
    //use strum::IntoEnumIterator;
    use strum_macros::EnumIter;
    #[repr(u8)] //use u8's explicitly
    #[derive(PartialEq, Debug,EnumIter)]
    pub enum ComponentTypeId {
        Null,
        Nosecone = 1,
        Bodytube = 2,
        InnerTube = 3,
        Fins = 4,
    }
    impl Display for ComponentTypeId {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::Null => write!(f,"Invalid Component"),
                Self::Nosecone => write!(f,"Nosecone"),
                Self::Fins =>write!(f,"Fins"),
                Self::InnerTube => write!(f,"Innertube"),
                Self::Bodytube => write!(f,"Bodytube"),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Component {
        name : String,
        typ : i8,
        props : HashMap<String, f64>,
    }
    impl Component {
        pub fn new() -> Self {
            Self{
                name : String::new(),
                typ : 0,
                props : HashMap::new(),
            }
        }
        pub fn has_prop (&self, key : &str) -> bool {
            return self.props.contains_key(key);
        }
        pub fn prop (&self, key : &str) -> f64 {
            return self.props[key];
        }
    }


    #[derive(Serialize, Deserialize, Debug)]
    pub struct Rocket {
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
        pub fn from_file (spath : String) -> Self {
            let contents = std::fs::read_to_string(spath)
            .expect("Check inputted path, it is incorrect or unavailible");
            return serde_json::from_str(&contents.to_string()).unwrap();
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
                println!("Name {} \n Prop. name: Prop. value", comp.name);
                for p in &comp.props {
                    println!("{tab}{};{}", p.0, p.1, tab=" ".repeat(indent))
                }
            }
            println!("--------------");
            self.precompute_mprops();
            println!("Major Properties \n Name: Value");
            for p in &self.precompute {
                println!("{tab}{} : {}", p.0, p.1, tab=" ".repeat(indent))
            }
            println!("--------------");
            self.hash();
            println!("prophash : {}",self.prop_hash)
        }
    }
}

mod cli;
fn main() {
    let ARGS = std::env::args(); //get command line args

    use crate::ModelRocket as MR;
    use crate::cli::Cli;
    let mut r = MR::Rocket::new();
    let mut c = Cli::default();
    ctrlc::set_handler(move || {cli::ctlc_handler();std::process::exit(0)}).expect("Error setting Ctrl-C handler");
    let test :f64  = c.promptf64("test");
    println!("{}",test);

    // println!("RSRocket CLI | version {} \n Input \"H\" for help.",0);
    // if let "h" | "H" = &cli::read() as & str {
    //     cli::dumpfile("./help/help.txt");
    // }
    // println!("Create new file or load?\n [N] [L]");
    // if let "n" | "N" = &cli::read() as &str {
    //     r = MR::Rocket::new();
    // } else if let "l" | "L" = &cli::read() as &str {
    //     println!("Enter file path: \n");
    //     r = MR::Rocket::from_file(cli::read());
    // }
    // cli::get_component_type();
    //application loop
    // loop{
    //     println!("Actions availible\n [A]dd component [R]emove component [D]isplay");
    //     if let "a" | "A" = &cli::read() as &str {
    //         println!("Component Types\n [N]osecone [T]ube");
    //         if let "n" | "N" = &cli::read() as &str {
    //             r.add_component(Component {
    //                 name : prompt("Component name:\n"),
    //                 typ : prompt::<i8>("Component type:\n"),
    //                 props : HashMap::from([
    //                     (String::from("diameter"), prompt::<f64>("Noscone base diameter:\n")),
    //                     (String::from("position"), prompt::<f64>("Nosecone position:\n")),
    //                     (String::from("length"), prompt::<f64>("Nosecone shape length:\n")),
    //                     (String::from("param"), prompt::<f64>("Nosecone shape parameter:\n")),
    //             ])})
                
    //         } else if let "t" | "T" = &read() as &str {
                
    //         }
    //     }
    //     if let "r" | "R" = &read() as &str {

    //     }
    //     if let "d" | "D" = &read() as &str {
            
    //     }
    // }
}