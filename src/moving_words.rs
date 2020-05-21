use crate::moving_word::MovingWord;

pub struct MovingWords {
    pub words: Vec<(String, String)>,
    pub moving: MovingWord,
    pub current_word: usize,
}

impl MovingWords {
    pub fn new(words_i: Vec<(String, String)>) -> Self {
        let mut new_item = MovingWords{words: words_i, moving: MovingWord::new("", ""), current_word: 0};
        new_item.moving.change_word(&new_item.words[0].0, &new_item.words[0].1);
        new_item
    }

    pub fn next(&mut self) {
        if self.current_word < (self.words.len() - 1) {
            self.current_word = self.current_word + 1;
            self.moving.change_word(&self.words[self.current_word].0, &self.words[self.current_word].1);
        }
    }

    pub fn prev(&mut self) {
        if self.current_word > 0 {
            self.current_word = self.current_word - 1;
            self.moving.change_word(&self.words[self.current_word].0, &self.words[self.current_word].1);
        }
    }
}