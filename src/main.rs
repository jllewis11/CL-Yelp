mod place;



fn main () {
    //A vector of place objects
    let mut places:Vec<place::Place> = Vec::new();
    //Create a place object
    let mut p1 = place::Place::new("Bowlero Cerritos".to_string(),
                        "18811 Carmenita Rd, Cerritos, CA 90703".to_string(), 
                        "Indoor".to_string(), 
                        "$$".to_string(), 
                        15,
                        Some("This place is great".to_string())
                        );
    //Push the place object into the vector
    places.push(p1);
    let mut p2 = place::Place::new("Shortyz".to_string(),
                        "-".to_string(), 
                        "Restaurant".to_string(), 
                        "$".to_string(), 
                        15,
                        Some("Great eats!".to_string())
                        ); 
    places.push(p2);                       
    //Print the vector of place objects
    for i in &places {
        i.print();
        println!("----------------------------------")
    }
    places[0].update_name("Bowlero".to_string());
    places[0].push("I love this place".to_string());
    places[0].print();
    println!("----------------------------------");
}

