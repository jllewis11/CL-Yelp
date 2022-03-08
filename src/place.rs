
pub struct Place{
    name: String, 
    address: String,
    coltype: String,
    rating: String,
    avgprice: u64,
    comment: Vec<String>,
}
impl Place{
    pub fn new(n:String, a:String, c:String, r:String, avg:u64, opt1:Option<String>) -> Place {
        let mut temp = Place {
            name: n,
            address: a,
            coltype: c,
            rating: r,
            avgprice: avg,
            comment: Vec::new(),
        };
        temp.push(opt1.unwrap());
        return temp;
    }
    pub fn push(&mut self, s:String) {
        self.comment.push(s);
    }
    pub fn print(&self) {
        println!("Name: {}", self.name);
        println!("Address: {}", self.address);
        println!("Coltype: {}", self.coltype);
        println!("Rating: {}", self.rating);
        println!("Avgprice: {}", self.avgprice);
        //display a vector of comments
        println!("Comments: ");
        for i in &self.comment {
            println!("{}", i);
        }
    }
    //update the name of the place object
    pub fn update_name(&mut self, s:String) {
        self.name = s;
    }
    //update the address of the place object
    pub fn update_address(&mut self, s:String) {
        self.address = s;
    }
    //update the coltype of the place object
    pub fn update_coltype(&mut self, s:String) {
        self.coltype = s;
    }
    //update the rating of the place object
    pub fn update_rating(&mut self, s:String) {
        self.rating = s;
    }
    //update the avgprice of the place object
    pub fn update_avgprice(&mut self, s:u64) {
        self.avgprice = s;
    }
    //Remove a comment based upon the position of the comment in the vector
    pub fn remove_comment(&mut self, pos:usize) {
        self.comment.remove(pos);
    }

}
