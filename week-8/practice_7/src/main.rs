fn main() {
    // initialize a mutable tuple 
    let mut mountain_heights = ("Everest",8848, "Fishtail", 6993);

    println!("Oringinal tuple = {:?}", mountain_heights);

    //change 3rd and 4th element of a mutable tuple 
    mountain_heights.2 ="Lhotse";
    mountain_heights.3 = 8516;

    println!("Changed tuple = {:?}", mmoountain_heights);
}