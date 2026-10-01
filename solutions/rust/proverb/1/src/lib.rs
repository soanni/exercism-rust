pub fn build_proverb(list: &[&str]) -> String {
    //    todo!("build a proverb from this list of items: {list:?}")
    let list_len = list.len();
    let mut i = 0;
    let mut j = 1;
    let mut proverb = String::new();
    if list_len == 0 {
        return proverb;
    }
    let end = format!("And all for the want of a {}.", list.get(0).unwrap());
    while i < list_len - 1 {
        let s = format!(
            "For want of a {} the {} was lost.\n",
            list.get(i).unwrap(),
            list.get(j).unwrap()
        );
        i += 1;
        j += 1;
        proverb.push_str(&s);
    }

    proverb.push_str(&end);
    proverb
}
