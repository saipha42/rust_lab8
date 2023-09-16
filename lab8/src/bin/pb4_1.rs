use std::io::Read;
use std::{env, process, io};
use clap::{App, Arg};
use std::fs::File;
use std::error::Error;
use csv::{Writer};
fn main() {

    let res = get_args();
    let points = load_points(res.0).unwrap();
    
    save_points(res.1, points);
}



fn get_args()-> (String, String) {

    let _matches = App::new("M APP")
    .about("This program is about reading and writing csv file")
    .arg(
        Arg::with_name("input")
        .value_name("Input File Name")
        .default_value("-")
    )
    .arg(
        Arg::with_name("output")
        .value_name("Output File Name")
        .default_value("-")
    ).get_matches();


    let in_filename = _matches.value_of("input").unwrap();
    let out_filename = _matches.value_of("output").unwrap();


    (in_filename.to_string(), out_filename.to_string())


    
}



fn load_points (file_path: String) -> io::Result<Vec<(String, String, String)>> {

    let mut result = Vec::new();
    
    if file_path == "-" {

        let mut input = String::from("");
        let csf_file = io::stdin().read_to_string(&mut input)?;
        

        for line in input.lines() {
            let mut pt = Vec::new();

            for word in line.split(",") {
                pt.push(word);
            }
            result.push( (pt[0].to_string(), pt[1].to_string(), pt[2].to_string()) );
        }
        Ok(result)

    }else{

        let csv_file = File::open(file_path).unwrap();
        let mut records = csv::Reader::from_reader(csv_file);

        for record in records.records() {
            
            if let Ok(rec) = record {
                let x : f32 = rec[0].parse().unwrap();
                let y: f32 = rec[1].parse().unwrap();
    
    
                let dst : f32 = (x*x + y*y).sqrt();
                result.push( ( x.to_string(), y.to_string(), String::from("#000000")) );
                
            }
        }
        Ok(result)
    }


}


fn save_points(filename: String ,pt_list : Vec<(String, String, String)>)-> Result<(), String> {

    if filename == "-" {
        for pt in pt_list {
            println!("{},{},{}", pt.0, pt.1, pt.2);
        }
        Ok(())

    }else {

        let mut writer = csv::Writer::from_path(filename).unwrap();

        for pt in pt_list {
            writer.write_record(vec![pt.0, pt.1, pt.2]);
        }
        writer.flush();
    
        Ok(())
    }

}