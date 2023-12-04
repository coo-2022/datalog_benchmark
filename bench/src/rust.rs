struct Table1 {
    key: i32,
    value: String,
}

struct Table2 {
    key: i32,
    value: String,
}

pub fn join_tables(table1: Vec<Table1>, table2: Vec<Table2>) -> Vec<(i32, String, String)> {
    let mut result = Vec::new();
    for t1 in table1.iter() {
        for t2 in table2.iter() {
            if t1.key == t2.key {
                result.push((t1.key, t1.value.clone(), t2.value.clone()));
            }
        }
    }
    result
}
