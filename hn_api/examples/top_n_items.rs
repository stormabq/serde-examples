use hn_api::HnClient;

fn top_n_items(numofitems: u32, max_number: u32) -> Vec<u32> {
    let mut items = Vec::new();
    items.push(max_number);
    let mut value = max_number;

    for _ in 1..numofitems {
        value = value - 1;
        items.push(value);
    }
    items
}

fn main() {
    let api = HnClient::init().unwrap();

    let max_item_id = api.get_max_item_id().unwrap();
    println!("max item id = {}", max_item_id);

    let item_ids = top_n_items(5, max_item_id);

    for i in &item_ids {
        println!("{}", i);
    }
}
