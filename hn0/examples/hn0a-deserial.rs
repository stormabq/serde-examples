use serde_json::from_str;
use spaceapi::Hna;

fn main() {
    // This is the original example
    // let input = r#"{"api":"0.13","space":"coredump","logo":"https://www.coredump.ch/logo.png","url":"https://www.coredump.ch/","location":{"lat":47.22936,"lon":8.82949},"contact":{"irc":"irc://freenode.net/#coredump","twitter":"@coredump_ch","foursquare":"525c20e5498e875d8231b1e5","email":"danilo@coredump.ch"},"issue_report_channels":["email","twitter"],"state":{"open":null},"ext_ccc":"chaostreff"}"#;

    // This is the final hackernews output
    // let input = r#"{"by":"dhouston","id":8863,"score":104,"time":1175714200,"title":"My YC app: Dropbox - Throw away your USB drive","type":"story","url":"http://www.getdropbox.com/u/2/screencast.html"}"#;

    // let input = r#"{"by":"dhouston","id":8863,"score":104,"time":1175714200,"title":"My YC app: Dropbox - Throw away your USB drive","type":"story","url":"http://www.getdropbox.com/u/2/screencast.html"}"#;

    let input = r#"{"by":"dbhouston","id":8863,"score":104,"title":"My Title","url":"http://www.zrato.com"}"#;

    let hna: Result<Hna, _> = from_str(input);
    match hna {
        Ok(hna) => println!("{:?}", hna),
        Err(err) => println!("Could not parse hna: {}", err),
    };
}
