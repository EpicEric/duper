use smol::{LocalExecutor, io::AsyncReadExt};

use crate::common::{get_query_output_reader, parse_duper_values};

#[test]
fn filter_range() {
    let query = r#"filter .http.history[..2] == "/" | format "${.id:raw}""#;
    let values = parse_duper_values(&[
        include_str!("../data/1.duper"),
        r#"{id:"2",http:{}}"#,
        r#"{id:"3",http:{history:{}}}"#,
        r#"{id:"4",http:{history:("/", "/", "/")}}"#,
        r#"{id:"5",http:{history:["/1", "/2", "/"]}}"#,
        r#"{id:"6",http:{history:["/1", "/", "/3"]}}"#,
        r#"{id:"7",http:{history:["/", "/2", "/3"]}}"#,
    ]);
    let executor = LocalExecutor::new();
    let (mut reader, fut) = get_query_output_reader(&executor, query, values);
    smol::block_on(async {
        executor.run(fut).await;
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await.unwrap();
        let ids: Vec<&str> = buf.trim().lines().collect();
        assert_eq!(ids, ["1", "6", "7"]);
    });
}
