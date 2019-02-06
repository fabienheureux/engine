pub trait Physics: std::fmt::Debug {
    fn move(&mut self, time: &Time);
}
