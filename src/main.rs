mod place;



fn main () {
    let mut p1 = place::Place::new("Bowlero Cerritos".to_string(),
                        "18811 Carmenita Rd, Cerritos, CA 90703".to_string(), 
                        "Indoor".to_string(), 
                        "$$".to_string(), 
                        15,
                        Some("This place is great".to_string())
                        );

    p1.print();
    println!("----------------------------------");
    p1.update("Bowlero".to_string());
    p1.push("I love this place".to_string());
    p1.print();
    println!("----------------------------------");
}

