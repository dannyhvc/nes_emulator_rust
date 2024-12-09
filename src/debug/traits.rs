/// Simple
pub trait RawRamViewDimensions<const WIDTH: usize, const HEIGHT: usize> {
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;

    fn first_n_words(&self) -> Vec<Vec<&u8>>;
    fn last_n_words(&self) -> Vec<Vec<&u8>>;
}
