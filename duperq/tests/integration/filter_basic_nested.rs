use smol::{LocalExecutor, io::AsyncReadExt};

use crate::common::{get_query_output_reader, parse_duper_values};

#[test]
fn filter_basic_nested() {
    let query = r#"filter .http.method == "GET" | format "${.id:raw}""#;
    let values = parse_duper_values(&[
        include_str!("../data/1.duper"),
        r#"{id:"2"}"#,
        r#"{id:"3",http:{}}"#,
        r#"{id:"4",http:{method:"POST"}}"#,
    ]);
    let executor = LocalExecutor::new();
    let (mut reader, fut) = get_query_output_reader(&executor, query, values);
    smol::block_on(async {
        executor.run(fut).await;
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await.unwrap();
        let ids: Vec<&str> = buf.trim().lines().collect();
        assert_eq!(ids, ["1"]);
    });
}
