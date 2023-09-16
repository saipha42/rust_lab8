


#[derive(Debug)]
struct  Point {
    x : f32,
    y : f32
}

#[derive(Debug)]
struct  TagPoint {
    x: f32,
    y: f32,
    color : String
}
fn main() {

    let pt_list = [
        Point {x : 0.0, y : 0.0},
        Point {x:0.0, y: 1.0},
        Point {x:1.0 , y: 1.0},
        Point {x: 0.5, y: 0.5}
    ];


    let res = tag_points(&pt_list);
    println!("{:#?}", res);


}


fn tag_points(pt_list : &[Point] )-> Vec<((f32, f32), String)> {

    let mut result = Vec::new();
    for i in 0..pt_list.len() {
        
        let pt = &pt_list[i];

        let dst : f32 = (pt.x* pt.x + pt.y * pt.y).sqrt();
        
        
        if dst <= 1.0 {
            result.push(  ((pt.x, pt.y),String::from("#80FF8080")));
        }else {
            result.push(  ((pt.x, pt.y),String::from("#FF808080")));
        }
    }
    result
}


#[test]
fn test_tag_points() {

    let pt_list = [Point{x: 0.5, y: 0.5}, Point {y: 0.7, x: 0.7}, Point {x: 1.0, y: 1.0}];
    let expected = vec![
        ((0.5, 0.5), "#80FF8080".to_string()),
        ((0.7, 0.7),"#80FF8080".to_string()),
        ((1.0, 1.0),"#FF808080".to_string())
    ];

    assert_eq!(tag_points(&pt_list), expected);

}