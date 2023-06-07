use redis::Commands;

#[test]
fn test_unlink_keys() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let keys: Vec<String> = con.keys("*").unwrap();

    for key in keys {
        println!("key: {key}");
        con.unlink::<_, ()>(key).unwrap();
    }
}
