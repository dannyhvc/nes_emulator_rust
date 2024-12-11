/// Simple
pub trait RawRamViewDimensions<
    const WIDTH: usize,
    const HEIGHT: usize,
    const START: usize,
    const END: usize,
>
{
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;
    const START: usize = START;
    const END: usize = END;

    fn first_n_words(&self) -> Vec<Vec<&u8>>;
    fn last_n_words(&self) -> Vec<Vec<&u8>>;
}

pub trait HexView {
    fn display(&self, words_list: Vec<Vec<&u8>>) -> String;
}
