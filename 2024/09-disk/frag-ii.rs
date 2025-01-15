use std::io;

#[derive(Debug)]
struct Block {
    inode: usize,
    size: usize,
}

#[derive(Debug)]
struct Diskmap {
    files: Vec<Block>,
    free: Vec<Block>,
}

fn parse_diskmap(line: &str) -> Diskmap {
    let mut files = vec![];
    let mut free = vec![];
    let mut inode = 0;

    for (i, size) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
        let size = size as usize;
        let block = Block { inode, size };
        match i % 2 {
            0 => files.push(block),
            _ => free.push(block),
        }
        inode += size;
    }

    Diskmap { files, free }
}

fn compact_disk(mut diskmap: Diskmap) -> Diskmap {
    for file in diskmap.files.iter_mut().rev() {
        for segment in diskmap.free.iter_mut() {
            if segment.inode >= file.inode { break };
            if file.size <= segment.size {
                file.inode = segment.inode;
                segment.inode += file.size;
                segment.size -= file.size;
                break;
            }
        }
    }
    diskmap
}

fn checksum(diskmap: &Diskmap) -> usize {
    diskmap
        .files
        .iter()
        .enumerate()
        .map(|(id, blk)| {
            (0..blk.size).map(|i| (blk.inode + i) * id).sum::<usize>()
        })
        .sum()
}

fn main() {
    let line = io::stdin().lines().next().unwrap().unwrap();
    println!("{}", checksum(&compact_disk(parse_diskmap(&line))));
}
