use std::{ fs::File, error::Error};


#[derive(Debug)]
struct  TagPoint {
    x: String,
    y: String,
    color : String
}
fn main() {

    // let pt_list = [
    //     Point {x : 0.0, y : 0.0},
    //     Point {x:0.0, y: 1.0},
    //     Point {x:1.0 , y: 1.0},
    //     Point {x: 0.5, y: 0.5}
    // ];

    if let Ok(res) = load_points(String::from("test.csv")) {
        println!("{:?}", res);
    }else {
        println!("Get error");
    }


}


fn load_points (file_path : String) -> Result<Vec<(String, String, String)>, Box<dyn Error>> {

    let mut result = Vec::new();
    let csv_file = File::open(file_path)?;
    let mut records = csv::Reader::from_reader(csv_file);

    for record in records.records() {
        
        if let Ok(rec) = record {
            let x : f32 = rec[0].parse().unwrap();
            let y: f32 = rec[1].parse().unwrap();


            let dst : f32 = (x*x + y*y).sqrt();
            result.push( (x.to_string(), y.to_string(), String::from("#000000")) );
            
        }
    }


    Ok(result)
}


#[test]
fn test_load_points() {
    let expected = vec![
        (0.0.to_string(), 0.0.to_string(), "#000000".to_string()),
        (1.5.to_string(), 0.0.to_string(), "#000000".to_string()),
        (0.5.to_string(), 0.5.to_string(), "#000000".to_string()),
    ];

    assert_eq!(expected, load_points(String::from("test.csv")).unwrap());

}