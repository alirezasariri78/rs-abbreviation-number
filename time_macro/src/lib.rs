pub trait TimeCalculator {
    fn get_year(&self) -> u64;
    fn get_month(&self) -> u64;
    fn get_day(&self) -> u64;
    fn get_hour(&self) -> u64;
    fn get_minute(&self) -> u64;
    fn get_second(&self) -> u64;
}
