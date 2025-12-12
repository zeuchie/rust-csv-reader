#[allow(unused)]

struct Track {
    track_id: String,
    track_name: String,
    track_number: u32,
    track_popularity: u32,
    explicit: bool,
    artist_name: String,
    artist_popularity: u32,
    artist_followers: u32,
    artist_genres: String,
    album_id: String,
    album_name: String,
    album_release_date: String,
    album_total_tracks: u32,
    album_type: String,
    track_duration_min: f32,
}

pub struct CSV {
    rows: Vec<Track>,
}

pub fn parse_csv(csv: &str) -> CSV {
    let mut final_csv = CSV { rows: Vec::new() };
    let mut lines = csv.lines();

    let _ = lines.next(); // Skip the first line in the CSV
    while let Some(line) = lines.next() {
        let mut columns = split_row(line).into_iter(); // Split the row into columns

        let track_id: String = columns.next().unwrap().parse().unwrap();
        let track_name: String = columns.next().unwrap().parse().unwrap();
        let track_number: u32 = columns.next().unwrap().parse().unwrap();
        let track_popularity: u32 = columns.next().unwrap().parse().unwrap();

        let explicit: bool = {
            let col = columns.next().unwrap();
            let explicit_boolean: bool = if col == "TRUE" { true } else { false };
            explicit_boolean
        };

        let artist_name: String = columns.next().unwrap().parse().unwrap();
        let artist_popularity: u32 = columns.next().unwrap().parse().unwrap();
        let artist_followers: u32 = columns.next().unwrap().parse().unwrap();
        let artist_genres: String = columns.next().unwrap().parse().unwrap();
        let album_id: String = columns.next().unwrap().parse().unwrap();
        let album_name: String = columns.next().unwrap().parse().unwrap();
        let album_release_date: String = columns.next().unwrap().parse().unwrap();
        let album_total_tracks: u32 = columns.next().unwrap().parse().unwrap();
        let album_type: String = columns.next().unwrap().parse().unwrap();
        let track_duration_min: f32 = columns.next().unwrap().parse().unwrap();

        let track = Track {
            track_id,
            track_name,
            track_number,
            track_popularity,
            explicit,
            artist_name,
            artist_popularity,
            artist_followers,
            artist_genres,
            album_id,
            album_name,
            album_release_date,
            album_total_tracks,
            album_type,
            track_duration_min,
        };

        final_csv.rows.push(track);
    }

    final_csv
}

fn split_row(s: &str) -> Vec<&str> {
    let mut row_as_vec: Vec<&str> = Vec::new();
    let mut s = s;

    while s != "" {
        // While string is not empty
        println!("{s}");
        if s.starts_with('"') {
            // String start has been found
            s = &s[1..]; // Skip the opening quote
            let quote_index = s.find("\",").or_else(|| s.find('"')).unwrap(); // Find the
            let substring = &s[0..quote_index];
            row_as_vec.push(substring);
            s = &s[quote_index + 1..]; // Move past the closing quote
            if s.starts_with(',') {
                s = &s[1..]; // Set s to everything after the comma following the closing quote
            }
        } else {
            // Else find next comma
            let comma_index = s.find(',');
            match comma_index {
                Some(comma_index) => {
                    // Found a comma
                    let substring = &s[0..comma_index];
                    row_as_vec.push(substring);
                    s = &s[comma_index + 1..]; // Set s to everything after the comma
                }
                None => {
                    // No more commas
                    row_as_vec.push(s);
                    break;
                }
            }
        }
    }

    return row_as_vec;
}

#[test]
fn test_split_row() {
    let input = "1,2,name,age,my"; // Simple case with only commas
    let expected = vec!["1", "2", "name", "age", "my"];
    assert_eq!(split_row(input), expected);

    let input = "1,2,name,age,\"my, name\""; // Column with string in quotes
    let expected = vec!["1", "2", "name", "age", "my, name"];
    assert_eq!(split_row(input), expected);

    let input = r#"3EJS5LyekDim1Tf5rBFmZl,"Trippy Mane (ft. Project Pat)",4,0,TRUE,Diplo,77,2812821,moombahton,5QRFnGnBeMGePBKF2xTz5z,"d00mscrvll, Vol. 1",2025-10-31,9,album,1.55"#;
    let expected = vec![
        "3EJS5LyekDim1Tf5rBFmZl",
        "Trippy Mane (ft. Project Pat)",
        "4",
        "0",
        "TRUE",
        "Diplo",
        "77",
        "2812821",
        "moombahton",
        "5QRFnGnBeMGePBKF2xTz5z",
        "d00mscrvll, Vol. 1",
        "2025-10-31",
        "9",
        "album",
        "1.55",
    ];
    assert_eq!(split_row(input), expected);

    let input = r#""nigerian drill, alté, afro adura, afrobeats, afrobeat, afroswing",3ARxksm8CspGeAaZZB1v2w,"LET'S GO!",2025-10-28,1,single,2.4"#;
    let expected = vec![
        "nigerian drill, alté, afro adura, afrobeats, afrobeat, afroswing",
        "3ARxksm8CspGeAaZZB1v2w",
        "LET'S GO!",
        "2025-10-28",
        "1",
        "single",
        "2.4",
    ];
    assert_eq!(split_row(input), expected);
}
