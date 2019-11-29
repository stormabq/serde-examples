use hn_api::HnClient;

fn print(api: &HnClient, items: &[u32]) {
    for id in items {

        // let item = api.get_item(*id).unwrap().unwrap();

        // let item = api.get_item(*id).to_string();

        let item = api.get_item(*id).unwrap().to_string();
        println!("{}",item);

/*
        let _err = match item {
                Some(item) => {
                    println!("Good to go")
                },
                None => {
                    println!("Got error")
                },
            };
*/



        // println!("{} {}", id, item.id());

        // println!("- {}: {}", item.id(), item.title().unwrap_or("?"),)
    }
}

fn main() {
    let api = HnClient::init().unwrap();

    // This is the call that returns a
    // working array
    // let top = api.get_top_stories().unwrap();

    // This works
    // let top = [21655958, 21656551, 21656190];
    // This breaks
    let top = [21655958,21656551,21654193];

    let count = 3;

    println!("Top {} stories:", count);
    print(&api, &top[..count]);
}
