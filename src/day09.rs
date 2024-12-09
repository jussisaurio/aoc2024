use crate::util::*;

pub fn day9_part1() -> usize {
    let mut input = aoc_read_day_bytes(9)
        .iter()
        .map(|&c| (c - b'0') as usize)
        .collect::<Vec<usize>>();

    // input has odd length so the first free block is at index 1 and the first file from end is at index len - 1
    let mut l = 0;
    let mut r = input.len() - 1;
    let mut compacted_disk = vec![];
    let mut fileid_l = 0;
    let mut fileid_r = input.len() / 2;
    while l <= r {
        if l % 2 == 0 {
            for _ in 0..input[l] {
                compacted_disk.push(fileid_l);
            }
            l += 1;
            fileid_l += 1;
        } else {
            let freespace = input[l];
            let space_to_take = input[r].min(input[l]);
            for _ in 0..space_to_take {
                compacted_disk.push(fileid_r);
            }
            if freespace > input[r] {
                input[l] -= input[r];
                input[r] = 0;
                r -= 2;
                fileid_r -= 1;
            } else {
                input[l] -= freespace;
                input[r] -= freespace;
                l += 1;
            }
        }
    }

    let mut sum = 0;
    for (pos, fileid) in compacted_disk.iter().enumerate() {
        sum += pos * fileid;
    }
    sum
}

#[derive(Clone, Copy)]

struct Block {
    file_id: Option<usize>,
    run_length: u8,
}

impl Block {
    fn is_empty(&self) -> bool {
        self.file_id.is_none()
    }

    fn can_fit(&self, other: &Block) -> bool {
        self.is_empty() && other.run_length <= self.run_length
    }
}

pub fn day9_part2() -> usize {
    let mut compacted_disk = aoc_read_day_bytes(9)
        .iter()
        .map(|&c| (c - b'0') as u8)
        .enumerate()
        .map(|(idx, count)| Block {
            file_id: if idx % 2 == 0 { Some(idx / 2) } else { None },
            run_length: count,
        })
        .collect::<Vec<Block>>();

    let mut cur_fileid_to_move = compacted_disk.len() / 2;
    let mut cur_fileidx_to_move = compacted_disk.len() - 1;
    while cur_fileid_to_move > 0 {
        let file_to_move = compacted_disk[cur_fileidx_to_move];
        if file_to_move.file_id == Some(cur_fileid_to_move) {
            for idx in 0..cur_fileidx_to_move {
                let target_candidate_block = &mut compacted_disk[idx];
                if target_candidate_block.can_fit(&file_to_move) {
                    let leftover_space =
                        target_candidate_block.run_length - file_to_move.run_length;
                    target_candidate_block.file_id = file_to_move.file_id;
                    target_candidate_block.run_length = file_to_move.run_length;
                    compacted_disk[cur_fileidx_to_move] = Block {
                        file_id: None,
                        run_length: file_to_move.run_length,
                    };
                    if leftover_space == 0 {
                        break;
                    }
                    let next_elem = &mut compacted_disk[idx + 1];
                    if next_elem.is_empty() {
                        // compact/modify
                        next_elem.run_length += leftover_space;
                    } else {
                        compacted_disk.insert(
                            idx + 1,
                            Block {
                                file_id: None,
                                run_length: leftover_space,
                            },
                        );
                        cur_fileidx_to_move += 1;
                    }

                    break;
                }
            }
            cur_fileid_to_move -= 1;
            cur_fileidx_to_move -= 2;
        } else {
            if cur_fileidx_to_move == 0 {
                cur_fileid_to_move -= 1;
                cur_fileidx_to_move = compacted_disk.len() - 1;
            } else {
                cur_fileidx_to_move -= 1;
            }
        }
    }

    let mut sum = 0;
    let mut actual_pos = 0;
    for block in compacted_disk.iter() {
        for _ in 0..block.run_length {
            sum += actual_pos * block.file_id.unwrap_or(0);
            actual_pos += 1;
        }
    }
    sum
}
