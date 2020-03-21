
pub struct Thing {
    pub obj: String,
}


impl Thing {
    pub fn do_the_thing(&self) {
        eprintln!("{}", self.obj);
    }
}
