use crepe::crepe;

crepe! {
    @input
    struct Table1(i32, String);

    @input
    struct Table2(i32, String);

    @output
    struct Joined(i32, String, String);

    Joined(key, v1, v2) <- Table1(key, v1), Table2(key, v2);
}

fn main() {
    let mut runtime = Crepe::new();
    // 假设 table1_data 和 table2_data 是数据
    for (key, value) in table1_data {
        runtime.extend(&[Table1(key, value)]);
    }
    for (key, value) in table2_data {
        runtime.extend(&[Table2(key, value)]);
    }
    let (joined,) = runtime.run();
    for j in joined {
        println!("{:?}", j);
    }
}
