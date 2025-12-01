pub trait Challenge {
    fn solve(&self, input: &str) -> Result<String, String>;
    fn solve_b(&self, input: &str) -> Result<String, String>;
}
