use hn_api::HnClient;

fn print(api: &HnClient, items: &[u32]) {
    for id in items {
        let item = api.get_item(*id).unwrap().unwrap();

        println!("{} {}", id, item.id());

        println!("- {}: {}", item.id(), item.title().unwrap_or("?"),)
    }
}

fn main() {
    let api = HnClient::init().unwrap();

    let top = api.get_top_stories().unwrap();

    let count = 3;

    println!("Top {} stories:", count);
    print(&api, &top[..count]);
}
