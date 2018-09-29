
use bio;

//use itertools::Itertools;

pub fn run(input_music: &mut Vec<i16>, input_sequence: &str, sample_rate: u32) {
    let reader_fasta = bio::io::fasta::Reader::from_file(input_sequence).unwrap();
    let all_seq = reader_fasta.records().flat_map(|x| x.unwrap().seq().to_vec());
    let genome = all_seq.collect::<Vec<u8>>();

    let gi16 = genome2i16(genome.as_slice());
    let mut genome_iter = gi16.iter().cycle();

    let mut n = genome_iter.next().unwrap();
    for (i, val) in input_music.iter_mut().enumerate() {
        if i % (sample_rate / 2) as usize == 0 {
            n = genome_iter.next().unwrap();
        }

        *val ^= n;
    }
}

fn genome2i16(g: bio::utils::TextSlice) -> Vec<i16> {
    let mut buffer: Vec<i16> = Vec::new();

    for i in g.chunks(4) {
        let mut zero: i16 = 0;
      
        if i.len() != 4 {
            break;
        }

        for j in 0..8 {
            if j % 2 == 0 {
                zero |= (0 as i16) << (7 - j) * 2;
            } else {
                zero |= (u822bit(i[j/2]) as i16) << (7 - j) * 2;
            }
        }
   
        buffer.push(zero);
    }

    return buffer;
}

fn u822bit(n: u8) -> u8 {
    return match n as char{
        'A' => 0,
        'T' => 1,
        'C' => 2,
        'G' => 3,
        _ => 0,
    };
}


