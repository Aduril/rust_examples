fn main() {
        let lines = [
            "12 Drummers Drumming",
            "11 Pipers Piping",
            "10 Lords a Leaping",
            "9 Ladies Dancing",
            "8 Maids a Milking",
            "7 Swans a Swimming",
            "6 Geese a Laying",
            "5 Golden Rings",
            "4 Calling Birds",
            "3 French Hens",
            "2 Turtle Doves",
            "a Partridge in a Pear Tree"
        ];

    let mut i = 0;
    while i < 12 {
        println!("On the {}. day of Christmas my true love sent to me:", i + 1);
        for number in (12 - i)..13 {
            let line = if number == 12 && i > 0 {
                [ "and", lines[number - 1]].join(" ")
            }
            else {
                lines[number - 1].to_string()
            };
            println!("{}", line);
        }
        i = i + 1;
    }
}
