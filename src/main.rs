mod csv;

use std::fs;
use std::io::prelude::*;

// Useful resource for learning rust standard library: https://docs.rs/rustc-std-workspace-std/latest/std/

fn main() {
    /*
    // Reading from a .txt file example:
    {
    // Create output.txt file
    fs::write("output.txt", "Hello, world!").unwrap();

    // Open output.txt and save to file variable, create string, read file to string
    let mut file = fs::File::open("output.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    // Split string s at whitespace, print each word on new line with debug formatting
    s.split_whitespace().for_each(|word| println!("{:?}", word));
    }
    */

    // Reading from the Spotify .csv:
    let mut spotify_data = fs::File::open("spotify_data.csv").unwrap();
    let mut csv_string = String::new();
    spotify_data.read_to_string(&mut csv_string).unwrap();

    // Parse the csv string into a CSV struct which holds a vector of tracks (rows)
    let spotify_csv: csv::CSV = csv::parse_csv(&csv_string);

    // Call method to print tracks by artist_name
    spotify_csv.print_tracks_by_artist_name("Tame Impala".to_string());
    println!();

    // Call method to print top 10 artists by follower count
    spotify_csv.print_top_artists_by_followers(10);
    println!();
    
    // Call method to print 10 number of tracks with duration at least 2.5 minutes and explicit
    spotify_csv.print_n_tracks_with_duration_atleast_and_explicit(20, 2.5, true);
}
