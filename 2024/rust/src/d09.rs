use std::iter;

mod part1 {
    pub struct Disk {
        files: Vec<usize>,
        free_spaces: Vec<usize>,
    }

    impl Disk {
        pub fn iter(&self) -> DiskIterator {
            DiskIterator {
                disk: self,
                files_iter: BlockRepeatIterator::new(0, &self.files),
                files_iter_rev: RevBlockRepeatIterator::new(self.files.len() - 1, &self.files),
                free_space_iter: BlockRepeatIterator::new(0, &self.free_spaces),
            }
        }
    }

    pub struct DiskIterator<'a> {
        disk: &'a Disk,

        files_iter: BlockRepeatIterator,
        files_iter_rev: RevBlockRepeatIterator,
        free_space_iter: BlockRepeatIterator,
    }

    impl<'a> Iterator for DiskIterator<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.files_iter.block_id == self.files_iter_rev.block_id
                && self.files_iter.index as isize > self.files_iter_rev.index
            {
                return None;
            }

            if let Some(id) = self.files_iter.next() {
                return Some(id);
            }

            if let Some(_) = self.free_space_iter.next() {
                loop {
                    if let Some(id) = self.files_iter_rev.next() {
                        return Some(id);
                    }
                    self.files_iter_rev = self.files_iter_rev.prev_block(&self.disk.files);
                }
            }

            self.free_space_iter = self.free_space_iter.next_block(&self.disk.free_spaces);
            self.files_iter = self.files_iter.next_block(&self.disk.files);
            self.next()
        }
    }

    struct BlockRepeatIterator {
        block_id: usize,
        block_size: usize,
        index: usize,
    }

    impl BlockRepeatIterator {
        fn new(block_id: usize, blocks: &[usize]) -> Self {
            Self {
                block_id,
                block_size: blocks[block_id],
                index: 0,
            }
        }

        fn next_block(&self, blocks: &[usize]) -> Self {
            Self::new(self.block_id + 1, blocks)
        }
    }

    impl Iterator for BlockRepeatIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.block_size {
                self.index += 1;
                Some(self.block_id)
            } else {
                None
            }
        }
    }

    struct RevBlockRepeatIterator {
        block_id: usize,
        index: isize,
    }

    impl RevBlockRepeatIterator {
        fn new(block_id: usize, blocks: &[usize]) -> Self {
            Self {
                block_id,
                index: blocks[block_id] as isize - 1,
            }
        }

        fn prev_block(&self, blocks: &[usize]) -> Self {
            Self::new(self.block_id - 1, blocks)
        }
    }

    impl Iterator for RevBlockRepeatIterator {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= 0 {
                self.index -= 1;
                Some(self.block_id)
            } else {
                None
            }
        }
    }

    pub fn parse(input: &str) -> Disk {
        Disk {
            files: input
                .trim()
                .chars()
                .step_by(2)
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
            free_spaces: input
                .trim()
                .chars()
                .skip(1)
                .step_by(2)
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let disk = part1::parse(input);
    let checksum: usize = disk.iter().enumerate().map(|(i, id)| i * id).sum();
    checksum as i64
}

mod part2 {
    use std::collections::HashMap;

    type Id = usize;
    type Size = usize;
    type Pos = usize;

    #[derive(Debug)]
    pub struct Disk {
        pub files: HashMap<Id, (Size, Pos)>,
        pub free_space_map: HashMap<Size, Vec<(Size, Pos)>>,
    }

    pub fn parse(input: &str) -> Disk {
        let mut files: HashMap<Id, (Size, Pos)> = HashMap::new();
        let mut free_space_map: HashMap<Size, Vec<(Size, Pos)>> = HashMap::new();

        let mut pos = 0;
        for (i, size) in input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .enumerate()
        {
            if i % 2 == 0 {
                let id = i / 2;
                files.insert(id, (size, pos));
                pos += size;
            } else {
                (1..=size).for_each(|s| free_space_map.entry(s).or_default().push((size, pos)));
                pos += size;
            }
        }

        Disk {
            files,
            free_space_map,
        }
    }

    pub fn fragment(mut disk: Disk) -> HashMap<Id, (Size, Pos)> {
        for id in (0..=*disk.files.keys().max().unwrap()).rev() {
            let &(size, pos) = disk.files.get(&id).unwrap();
            if let Some(positions) = disk.free_space_map.get_mut(&size) {
                if let Some(&(free_size, free_pos)) =
                    positions.first().filter(|&&(_, free_pos)| free_pos < pos)
                {
                    disk.files.insert(id, (size, free_pos));

                    (1..=free_size).for_each(|s| {
                        disk.free_space_map.entry(s).and_modify(|positions| {
                            let new_free_size = free_size - size;
                            if s <= new_free_size {
                                positions
                                    .iter_mut()
                                    .find(|x| *x == &(free_size, free_pos))
                                    .map(|free_space| {
                                        *free_space = (new_free_size, free_pos + size);
                                    });
                            } else {
                                positions.remove(
                                    positions
                                        .iter()
                                        .position(|x| x == &(free_size, free_pos))
                                        .unwrap(),
                                );
                            };
                        });
                    });
                };
            };
        }

        disk.files
    }
}

pub fn part2(_input: &str) -> i64 {
    let disk = part2::parse(_input);

    let files = part2::fragment(disk);

    let checksum: usize = files
        .iter()
        .flat_map(|(&id, &(size, pos))| iter::repeat(id).zip(pos..pos + size))
        .map(|(id, pos)| id * pos)
        .sum();

    checksum as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2333133121414131402";
        assert_eq!(part1(input), 1928);
    }

    #[test]
    fn test_part2() {
        let input = "2333133121414131402";
        assert_eq!(part2(input), 2858);
    }
}
