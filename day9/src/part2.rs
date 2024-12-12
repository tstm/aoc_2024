#![allow(dead_code, unused_variables)]

#[derive(Debug)]
struct Block {
    length: u8,
    id: Option<usize>,
}

fn get_last_block(disk: &Vec<Block>, id: usize) -> Option<usize> {
    disk.iter()
        .enumerate()
        .filter(|(i, b)| b.id == Some(id))
        .map(|(i, b)| i)
        .next()
}

fn get_first_free(disk: &Vec<Block>, length: u8) -> Option<usize> {
    disk.iter()
        .enumerate()
        .filter(|(i, b)| b.id == None && b.length >= length)
        .map(|(i, b)| i)
        .next()
}

fn print(disk: &Vec<Block>) {
    for block in disk.iter() {
        match block.id {
            Some(v) => {
                for _ in 0..block.length {
                    print!("{}", v);
                }
            }
            None => {
                for _ in 0..block.length {
                    print!("N");
                }
            }
        }
    }
    println!();
}

pub fn run(input: &str) -> Result<usize, String> {
    let mut disk = vec![];
    let input = input.trim().as_bytes();

    let mut id: usize = 0;
    for i in 0..(input.len().div_ceil(2)) {
        let file_length: u8 = input.get(i * 2).unwrap() - 48;
        let free_length: u8 = input.get(i * 2 + 1).unwrap_or(&48u8) - 48;
        disk.push(Block {
            length: file_length,
            id: Some(id),
        });
        disk.push(Block {
            length: free_length,
            id: None,
        });

        // id = (id + 1) % 10;
        id = id + 1;
    }

    // Reset id
    id -= 1;

    while let Some(last_index) = get_last_block(&disk, id) {
        match get_first_free(&disk, disk[last_index].length) {
            Some(free_index) => {
                if free_index < last_index {
                    let free_length = disk[free_index].length;
                    let last_length = disk[last_index].length;

                    if free_length > last_length {
                        let last_id = disk[last_index].id.clone();
                        {
                            let free = disk.get_mut(free_index).unwrap();
                            free.length = free.length - last_length;
                            let last = disk.get_mut(last_index).unwrap();
                            last.id = None;
                        }
                        disk.insert(
                            free_index,
                            Block {
                                id: last_id,
                                length: disk[last_index].length,
                            },
                        );
                    } else if free_length == last_length {
                        {
                            disk.swap(free_index, last_index);
                        }
                    }
                }
            }
            None => {}
        }
        id -= 1;
        if id == 0 {
            break;
        }
    }

    // print(&disk);

    let mut checksum = 0;
    let mut index = 0;
    for block in disk {
        match block.id {
            Some(b) => {
                for i in 0..block.length {
                    checksum += index * b;
                    index += 1;
                }
            }
            None => {
                for i in 0..block.length {
                    index += 1;
                }
            }
        }
    }

    Ok(checksum)
}
