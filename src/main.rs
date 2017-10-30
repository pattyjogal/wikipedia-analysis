extern crate quick_xml;
extern crate regex;

use quick_xml::reader::Reader;
use quick_xml::events::Event;

use regex::Regex;

fn main() {
    // Defining the file reader (note this is relative to the top level
    // directory; make sure you call `cargo run` from there!
    let mut reader = Reader::from_file("./src/wiki1.xml").unwrap();
    // Vectors for the buffer and the found numbers
    let mut buf = Vec::new();
    let mut nums = Vec::new();
    // The regex pattern to match
    // TODO: Allow for commas
    let re = Regex::new(r"\d+([\d,]?\d)*(\.\d+)?").unwrap();

    loop {
        // When the reader encounters some read event,
        match reader.read_event(&mut buf) {
            // If it's at a start tag, ensure it's a <text> tag 
            Ok(Event::Start(ref e)) if e.name() == b"text" => {
                // Unwrap the actual text
                let text =  reader.read_text(b"text", &mut Vec::new()).unwrap();
                // Run the text against the regex pattern   
                match re.captures(text.as_str()) {
                    Some(res) => {
                        println!("{:?}", res); // Debugging
                        for group in res.iter() {
                            // Ensure that the capture group got a value
                            let s = match group {
                                Some(string) => string,
                                _ => continue
                            };
                            // Try to get an integer from that value
                            let n = match s.as_str().parse::<i32>() {
                                Ok(num) => num,
                                Err(num) => continue                            
                            };
                            // Only add numbers over 10^5
                            if n > 10000 {
                                nums.push(n);
                            }
                        }
                    },
                    _ => ()
                }
            }
            // Break at the end of file
            Ok(Event::Eof) => break,
            _ => ()
        }
        buf.clear();
    }

    println!("{:?}", nums);
}

