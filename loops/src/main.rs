fn main() {
    let day: [&str; 12] = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth", "Eleventh", "Twelfth"];
    let gifts: [&str; 12] = ["A Partridge in a Pear Tree", "Two Turtle Doves", "Three French Hens", "Four Calling Birds", "Five Gold Rings", "Six Geese a Laying", "Seven Swans a Swimming", "Eight Maids a Milking", "Nine Ladies Dancing", "Ten Lords a Leaping", "Eleven Pipers Piping", "Twelve Drummers Drumming"];
    
    for iteration in 0..12{
       print!("On the {} of Christmas My True Love Gave To Me,",
        day[iteration]);
        //println!("{}", iteration);
        for iteration2 in (0..=iteration).rev(){
            //println!("{}", iteration2);
            if iteration2 == 0 && iteration2 == iteration{
                print!(" {}", gifts[iteration2]);
            }
            else if iteration2 == 0{
                print!(" and {}", gifts[iteration2]);
            }
            else{
                print!(" {},", gifts[iteration2]);
            }
        }
       print!("\n\n")
    }
}