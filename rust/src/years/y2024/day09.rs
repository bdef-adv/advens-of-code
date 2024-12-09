#[derive(Debug)]
struct Block {
    file_id: u64,
    space: bool
}
impl Clone for Block {
    fn clone(&self) -> Self {
        Block {
            file_id: self.file_id,
            space: self.space,
        }
    }
}

#[derive(Debug)]
struct FileBlock {
    file_id: u64,
    space: bool,
    size: u8
}
impl Clone for FileBlock {
    fn clone(&self) -> Self {
        FileBlock {
            file_id: self.file_id,
            space: self.space,
            size: self.size
        }
    }
}

#[derive(Debug)]
struct Filesystem {
    raw_blocks: Vec<Block>,
    fs: Vec<FileBlock>,
    last_file_position: usize
}

impl Filesystem {
    fn from(input: &str) -> Filesystem {
        let mut raw_blocks: Vec<Block> = vec![];
        let mut fs: Vec<FileBlock> = vec![];

        let mut file_id: u64 = 0;
        let mut space = false;
        let mut last_file_position: usize = 0;
        for ch in input.chars() {
            let size: u8 = ch.to_digit(10).unwrap() as u8;
            raw_blocks.extend(vec![Block {file_id, space}; size as usize]);
            fs.push(FileBlock {file_id, space, size});
            if !space {
                last_file_position = raw_blocks.len() - 1;
                space = true;
                file_id += 1;
            } else {
                space = false;
            }
        }

        return Filesystem {
            raw_blocks,
            fs,
            last_file_position
        };
    }

    fn find_last_block(&mut self) {
        for (i, block) in self.raw_blocks.iter().rev().enumerate() {
            if !block.space {
                self.last_file_position = self.raw_blocks.len() - 1 - i;
                return;
            }
        }
    }

    fn is_raw_blocks_valid(&self) -> bool {
        let mut space_found = false;
        for block in self.raw_blocks.iter() {
            if !block.space && space_found {
                return false;
            }
            if block.space {
                space_found = true;
            }
        }
        return true;
    }

    fn move_raw_blocks(&mut self) {
        while !self.is_raw_blocks_valid() {
            for (position, block) in self.raw_blocks.iter().enumerate() {
                if block.space {
                    self.raw_blocks.swap(position, self.last_file_position);
                    break;
                }
            }
            self.find_last_block();
        }
    }

    fn find_first_available_space(&mut self, size: u8, max_index: usize) -> Option<usize> {
        for (pos, fileblock) in self.fs.iter().enumerate() {
            if pos >= max_index {
                return None;
            }
            if fileblock.space && fileblock.size >= size {
                return Some(pos);
            }
        }
        None
    }

    fn get_all_files(&self) -> Vec<(usize, FileBlock)> {
        let mut files: Vec<(usize, FileBlock)> = vec![];

        for (index, file) in self.fs.iter().rev().enumerate() {
            if !file.space {
                files.push((self.fs.len() - 1 - index, file.clone()));
            }
        }

        files
    }

    fn update_raw_blocks_from_files(&mut self) {
        self.raw_blocks = vec![];
        for block in self.fs.iter() {
            self.raw_blocks.extend(vec![Block {
                file_id: block.file_id,
                space: block.space
            }; block.size as usize]);
        }
    }

    fn move_file_blocks(&mut self) {
        let all_files: Vec<(usize, FileBlock)> = self.get_all_files();
        let files_count = all_files.len();
        let mut file_passed: Vec<u64> = vec![];
        let mut fs_len = self.fs.len();

        while file_passed.len() != files_count {
            let mut file_insert: FileBlock = FileBlock {file_id: 0, space: true, size: 0};
            let mut insert_position: i32 = -1;
            for (index, fileblock) in self.fs.clone().iter().rev().enumerate() {
                if fileblock.space || file_passed.contains(&fileblock.file_id) {
                    continue;
                }

                insert_position = -1;
                file_passed.push(fileblock.file_id);
                if let Some(position) = self.find_first_available_space(fileblock.size, fs_len - 1 - index) {
                    file_insert.size = self.fs[position].size - fileblock.size;

                    self.fs[position].file_id = fileblock.file_id;
                    self.fs[position].space = false;
                    self.fs[position].size = fileblock.size;

                    insert_position = position as i32 + 1;

                    self.fs[fs_len - 1 - index].space = true;
                    break;
                }
            }

            if insert_position > 0 {
                self.fs.insert(insert_position as usize, file_insert);
                fs_len = self.fs.len();
            }
        }
    }

    fn checksum(&self) -> u64 {
        let mut checksum: u64 = 0;

        for (index, block) in self.raw_blocks.iter().enumerate() {
            if !block.space {
                checksum += index as u64 * block.file_id;
            }
        }

        return checksum;
    }
}

fn solve_day(file_contents: &str) -> (u64, u64) {
    /*
        Part 1
     */
    let mut fs: Filesystem = Filesystem::from(file_contents);
    fs.move_raw_blocks();
    let sum_part1: u64 = fs.checksum();

    fs.move_file_blocks();
    fs.update_raw_blocks_from_files();

    let sum_part2: u64 = fs.checksum();
    
    (sum_part1, sum_part2)
}


pub fn get_day_results(file_contents: &str) -> (String, String) {
    /*
        Return this day's results as a tuple
     */
    let results = solve_day(&file_contents);
    (format!("{}", results.0), format!("{}", results.1))
}
