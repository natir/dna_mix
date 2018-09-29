
use bio;
use rand::{Rng, StdRng};

use std;
use std::collections::HashMap;

pub fn run(input_music: &mut Vec<i16>, input_assembly: &str) {

    let mut rng = StdRng::new().unwrap();
    let tig2len = read_assembly(input_assembly);
    
    let tig2wav = assign_wavpart_to_tig(input_music, &tig2len);
    
    let mut new_tig_order: Vec<&String> = tig2len.keys().collect();
    rng.shuffle(&mut new_tig_order);

    let mut cursor = 0;
    for tig in new_tig_order {
        let new_cursor = cursor + tig2wav[tig].len();
        input_music.splice(cursor..new_cursor, tig2wav[tig].iter().cloned());
        cursor = new_cursor
    }
}

fn assign_wavpart_to_tig(input_music: &mut Vec<i16>, tig2len: &HashMap<String, usize>) -> HashMap<String, Vec<i16>> {
    let mut tig2wav: HashMap<String, Vec<i16>> = HashMap::new();

    let total: usize = tig2len.values().sum();

    let factor = input_music.len() / total;

    let mut cursor = 0;
    for (tig, len) in tig2len.iter() {
        let new_pos = cursor + len * factor;
        tig2wav.insert(tig.to_string(), input_music[cursor..new_pos].to_vec());
        cursor = new_pos;
    }

    return tig2wav;
}

fn read_assembly(input_sequence: &str) -> HashMap<String, usize> {
    let mut tig2len = std::collections::HashMap::new();
    
    let reader = bio::io::fasta::Reader::from_file(input_sequence).unwrap();
    for r in reader.records() {
        let record = r.unwrap();
        tig2len.insert(record.id().to_string(), record.seq().len());
    }

    return tig2len;
}
