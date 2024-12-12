#![allow(dead_code, unused_variables)]

// use num::integer::Integer;

fn get_last_number(disk: &Vec<Option<usize>>, end_offset: usize) -> Option<usize> {
    for i in (0..end_offset).rev() {
        match disk[i] {
            Some(_) => return Some(i),
            None => {}
        }
    }
    None
}

fn get_first_free(disk: &Vec<Option<usize>>, start_offset: usize) -> Option<usize> {
    for i in start_offset..disk.len() {
        match disk[i] {
            Some(_) => {}
            None => return Some(i),
        }
    }
    None
}

fn print(disk: &Vec<Option<usize>>) {
    for value in disk.iter() {
        match value {
            Some(v) => print!("{}", v),
            None => print!("N"),
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
        for _ in 0..file_length {
            disk.push(Some(id));
        }
        for _ in 0..free_length {
            disk.push(None);
        }

        // id = (id + 1) % 10;
        id = id + 1;
    }

    let mut start_offset = 0;
    let mut end_offset = disk.len();

    loop {
        start_offset = match get_first_free(&disk, start_offset) {
            Some(n) => n,
            None => break,
        };
        end_offset = match get_last_number(&disk, end_offset) {
            Some(n) => n,
            None => break,
        };
        if start_offset < end_offset {
            disk.swap(start_offset, end_offset);
        } else {
            break;
        }
    }
    // print(&disk);

    let mut checksum = 0;
    for (i, value) in disk.into_iter().enumerate() {
        match value {
            Some(v) => checksum += i * v as usize,
            None => break,
        }
    }

    Ok(checksum)
}
