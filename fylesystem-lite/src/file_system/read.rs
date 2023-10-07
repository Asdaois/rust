use super::File;

impl File {
    pub fn read(&self, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        read_length
    }
}
