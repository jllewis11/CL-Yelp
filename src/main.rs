struct Place{
    name: String, 
    address: String,
    coltype: String,
    rating: String,
    avgprice: u64,
    comment: Vec<String>,

}
impl Place {
    fn push(&mut self, s:String) {
        self.comment.push(s);
    }
    fn print(&self) {
        println!("{}", self.name);
        println!("{}", self.address);
        println!("{}", self.coltype);
        println!("{}", self.rating);
        println!("{}", self.avgprice);
        for i in &self.comment{
            println!("{}", i);
        }
    }
    //update the name of the place object
    fn update (&mut self, s:String) {
        self.name = s;
    }
}


fn main () {
    let p1 = Place {
        name: String::from("Bowlero Cerritos"),
        address: String::from("18811 Carmenita Rd, Cerritos, CA 90703"),
        coltype: String::from("Indoor"),
        rating: String::from("$$"),
        avgprice: 15,
        comment: Vec::new(),
    };
    p1.print();
}

