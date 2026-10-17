use std::env;
use std::path::Path;
use std::io::Result;
use std::fs;
//use std::fmt::Error;
use std::io::Error;


// test data path is in gitignore
fn main() -> Result<()>{
    let args:Vec<String> = env::args().collect(); // so that it can be called from the terminal


    if args.len() <= 1{
        eprintln!("Usage : lmc <file.txt>");
        std::process::exit(1);
    }else{
        

        let file_path:&Path = Path::new(&args[1]);
        let contents:String = get_file(file_path)?; //get text

        let contents:Vec<&str> = contents.lines().collect();

        let sanitised:Vec<Vec<&str>> = sanitise(contents)?;
        println!("{:?}",sanitised);
        first_pass(sanitised);
    }
    

    Ok(())
}

fn get_file(name:&Path)->Result<String>{
    let mut contents:String = String::new();

    if fs::exists(name)?{
    
        contents = fs::read_to_string(name)?;
    }else{
        eprintln!("usage: lmc <file.txt>");
        std::process::exit(1);
    }
    Ok(contents)
}




fn sanitise(contents:Vec<&str>) -> Result<Vec<Vec<&str>>>{
    let mut sanitised:Vec<Vec<&str>> = Vec::new(); 

    for x in 0..contents.len(){
        let line:Vec<&str> = contents[x].trim_start().trim().split_whitespace().collect();
        
        if line.len() != 0{
            sanitised.push(line);
        }
    }

    Ok(sanitised) // result has been tokenized
}


fn first_pass(sanitised:Vec<Vec<&str>>)-> Result<()>{
    //let Opcodes:Vec<&str> = vec!["INP","OUT","LDA","STA","ADD","SUB","BRA","BRZ","BRP","HLT","DAT"];
    //let mut labels:Vec<&str> = Vec::new();
    //let mut dat_statements:Vec<&str> = Vec::new();
    let mut instructions:Vec<Instruction> =Vec::new();


    // go through each line and determine what is a label and what is an opcode

    for line in sanitised{
        println!("{:?}", line);
        for tok in line{
            let tokop:Opcode = Opcode::from_str(tok)?;
            
            let cur_line:Instruction = match tokop{
                Opcode::Dat => struct_dat(tokop , line),


            }

            

        }  
        instructions.push(cur_line);
    }

    Ok(())
}


fn struct_dat(tokop:Opcode ,line:Vec<&str>) -> Instruction{
    // LABEL DAT VALUE, dat statement format

    if line.len() == 2{
        let value = 0;
        let dat:Instruction<` a > ={
            label : line[0],
            opcode: tokop,
            operand: &"0"

        }

    }


}




#[derive(Debug, Clone, Copy, PartialEq)] // can use .clone() and doesn't move values and can compare vals
enum Opcode { Inp, Out, Lda, Sta, Add, Sub, Bra, Brz, Brp, Hlt, Dat} // type Opcode 
impl Opcode{
    fn from_str(opcode:&str) -> Result<Self, std::io::Error>{
        match opcode{
            "INP" => Ok(Opcode::Inp),
            "OUT" => Ok(Opcode::Out),
            "LDA" => Ok(Opcode::Lda),
            "STA" => Ok(Opcode::Sta),
            "ADD" => Ok(Opcode::Add),
            "SUB" => Ok(Opcode::Sub),
            "BRA" => Ok(Opcode::Bra),
            "BRZ" => Ok(Opcode::Brz),
            "BRP" => Ok(Opcode::Brp),
            "HLT" => Ok(Opcode::Hlt),
            "DAT" => Ok(Opcode::Dat),
            _ => Err(std::fmt::Error),

        }
    }
}

#[derive(Debug)]
struct Instruction<'a, > { // 'a is a lifetime parameter, ensures that the reference doesnt out live the data
    label: Option<&'a str>,    
    opcode: Opcode,
    operand: Option<&'a str>,
}


