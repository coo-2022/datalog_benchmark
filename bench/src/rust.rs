pub fn join_tables(table1: Vec<(i32, u64)>, table2: Vec<(i32, u64)>) -> Vec<(i32, u64, u64)> {
    let mut result = Vec::new();
    for t1 in table1.iter() {
        for t2 in table2.iter() {
            if t1.0 == t2.0 {
                result.push((t1.0, t1.1, t2.1));
            }
        }
    }
    result
}
