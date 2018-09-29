extern crate hound;
extern crate bio;
extern crate clap;
extern crate rand;

mod mix;
mod noise;

use clap::{App, Arg};

fn main() {
    let matches = App::new("dna_mix")
        .version("0.1 Mew")
        .author("Pierre Marijon <pierre@marijon.fr>")
        .about("Mix dna information with wav file")
        .arg(Arg::with_name("input")
             .short("i")
             .long("input")
             .required(true)
             .display_order(10)
             .takes_value(true)
             .help("input file in wav format")
            )
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .required(true)
             .display_order(15)
             .takes_value(true)
             .help("output file in wav format")
            )
        .arg(Arg::with_name("noise")
             .short("n")
             .long("noise")
             .display_order(20)
             .takes_value(true)
             .help("add noise to the input")
            )
        .arg(Arg::with_name("mix")
             .short("m")
             .long("mix")
             .display_order(30)
             .takes_value(true)
             .help("map input against fasta fragement and ")
            )
        .get_matches();

    let mut reader = hound::WavReader::open(matches.value_of("input").unwrap()).unwrap(); 
    let mut writer = hound::WavWriter::create(matches.value_of("output").unwrap(), reader.spec()).unwrap();

    let mut music = reader.samples::<i16>().map(|x| x.unwrap()).collect::<Vec<i16>>();

    if matches.is_present("noise") {
        noise::run(&mut music, matches.value_of("noise").unwrap(), reader.spec().sample_rate);
    }
    if matches.is_present("mix") {
        mix::run(&mut music, matches.value_of("mix").unwrap());
    }

    for sample in music {
        writer.write_sample(sample).unwrap();
    }
}


// graph d'assemblage suivre chemins eulériens

