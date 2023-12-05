use crepe::crepe;

crepe! {
    @input
    struct Table1(i32, u64);

    @input
    struct Table2(i32, u64);

    @output
    #[derive(Debug)]
    struct Joined(i32, u64, u64);

    Joined(key, v1, v2) <- Table1(key, v1), Table2(key, v2);
}

pub fn join_tables(table1_data: Vec<(i32, u64)>, table2_data: Vec<(i32, u64)>) {
    let mut runtime = Crepe::new();
    // 假设 table1_data 和 table2_data 是数据
    for (key, value) in table1_data {
        runtime.extend(&[Table1(key, value)]);
    }
    for (key, value) in table2_data {
        runtime.extend(&[Table2(key, value)]);
    }
    let (joined,) = runtime.run();
}
