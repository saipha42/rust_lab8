use std::{ fs::File, error::Error};
use csv::{Writer};


#[derive(Debug)]
struct  TagPoint {
    x: String,
    y: String,
    color : String
}
fn main() {

    let pt_list = run().unwrap();


    let mut writer = csv::Writer::from_path("out.csv").unwrap();

    save_points(&mut writer, pt_list);

}


fn run () -> Result<Vec<(String, String, String)>, Box<dyn Error>> {

    let mut result = Vec::new();
    let csv_file = File::open("test.csv")?;
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

fn save_points(writer : &mut Writer<File>, pt_list : Vec<(String, String, String)>)-> Result<(), String> {

    for pt in pt_list {
        writer.write_record(vec![pt.0, pt.1, pt.2]);
    }
    writer.flush();

    Ok(())
}


#[test]
fn test_save_points () {

    let pt_list = run().unwrap();
    let mut writer = csv::Writer::from_path("test_3_3.csv").unwrap();

    assert!(save_points(&mut writer, pt_list).is_ok());
}