fn main() {
    let ordinal_numbers_array: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let lyrics_array: [&str; 12] = [
        "And a partridge in a pear tree\n",
        "Two turtledoves\n",
        "Three French hens\n",
        "Four calling birds\n",
        "Five gold rings (five golden rings)\n",
        "Six geese a-laying\n",
        "Seven swans a-swimming\n",
        "Eight maids a-milking\n",
        "Nine ladies dancing\n",
        "Ten lords a-leaping\n",
        "Eleven pipers piping\n",
        "Twelve drummers drumming\n",
    ];

    let mut song_lines: Vec<String> = Vec::new();

    for i in 0..lyrics_array.len() {
        let mut line = String::new();
        for j in (0..=i).rev() {
            line.push_str(lyrics_array[j]);
        }
        song_lines.push(line);
    }

    println!(
        "On the first day of Christmas, my true love sent to me\nA partridge in a pear tree\n"
    );

    for i in 1..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            ordinal_numbers_array[i]
        );
        println!("{}", song_lines[i]);
    }

    println!("And a partridge in a pear tree");
}
