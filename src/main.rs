fn main() {
    sing_a_song();
}

fn sing_a_song() {
    for number in 0..12 {
        nth_day(number)
    }
}

fn nth_day(number: u32) {
    let ordinal = [
        "first", "second", "therd", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    println!(
        "On the {} day of Christmas my true love sent to me",
        ordinal[number as usize]
    );
    let mut not_first_flag = false;

    for day in (0..number + 1).rev() {
        if not_first_flag {
            if day == 0 {
                println!("{}.", gifts[day as usize]);
            } else {
                println!("{},", gifts[day as usize]);
            }
        } else {
            if day == 0 {
                println!("A partridge in a pear tree.");
            } else {
                println!("{},", gifts[day as usize]);
            }
        }

        not_first_flag = true;
    }

    println!();
}
