use redis::Commands;

#[test]
#[ignore = "destructive helper: deletes Redis keys and must be run explicitly"]
fn test_unlink_keys() {
    if std::env::var("SOMES_ALLOW_UNLINK_KEYS").as_deref() != Ok("1") {
        panic!("set SOMES_ALLOW_UNLINK_KEYS=1 to run this destructive Redis cleanup helper");
    }

    let redis_url =
        std::env::var("SOMES_TEST_REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_connection().unwrap();

    let keys: Vec<String> = con.keys("*").unwrap();

    for key in keys {
        println!("key: {key}");
        con.unlink::<_, ()>(key).unwrap();
    }
}
