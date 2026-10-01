use std::collections::HashMap;

pub fn raindrops(n: u32) -> String {
    // let mut res = String::new();
    let mut m: HashMap<u32, &str> = HashMap::new();
    m.insert(3, "Pling");
    m.insert(5, "Plang");
    m.insert(7, "Plong");

    //[3, 5, 7].into_iter().for_each(|k| {
    //    if n % k == 0 {
    //        res.push_str(m.get(&k).unwrap());
    //    }
    //});

    let mut res = [3, 5, 7]
        .into_iter()
        .map(|k| if n % k == 0 { m.get(&k).unwrap() } else { "" })
        .collect::<Vec<&str>>()
        .join("");

    if res.is_empty() {
        res.push_str(&n.to_string());
    }

    res
}
