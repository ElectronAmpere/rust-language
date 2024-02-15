// To print the lyrics of christmas carol

fn main()
{
	let carol_day = [
		"first",
		"second",
		"third",
		"fourth",
		"fifth",
		"sixth",
		"seventh",
		"eighth",
		"ninth",
		"tenth",
		"eleventh",
		"twelfth",
	];

    let carol_lyrics = [
		"A partridge in a pear tree",
		"Two turtle doves and",
		"Three french hens",
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

    for verse in 1..=12 {
		println!("Verse {}:", verse);
		println!("On the {} day of christmas, my true love sent me", carol_day[verse - 1]);
	for lyric in (0..verse).rev() {
	    println!("{}", carol_lyrics[lyric]);
	}
	println!("\n");
    }
}
