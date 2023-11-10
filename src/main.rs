mod timespan;
fn main() {

    let event = loop {
        match timespan::read_event() {
            Ok(event) => break event,
            Err(e) => {
                println!("{e}");
            }
        }
    };


}

