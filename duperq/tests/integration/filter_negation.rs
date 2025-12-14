use smol::{LocalExecutor, io::AsyncReadExt};

use crate::common::{get_query_output_reader, parse_duper_values};

#[test]
fn filter_negation() {
    let query = r#"filter !.development | format "${.id}""#;
    let values = parse_duper_values(&[
        include_str!("../data/1.duper"),
        r#"{id:"2"}"#,
        r#"{id:"3",development:{}}"#,
        r#"{id:"4",development:0}"#,
        r#"{id:"5",development:true}"#,
        r#"{id:"6",development:10}"#,
    ]);
    let executor = LocalExecutor::new();
    let (mut reader, fut) = get_query_output_reader(&executor, query, values);
    smol::block_on(async {
        executor.run(fut).await;
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await.unwrap();
        let ids: Vec<&str> = buf.trim().lines().collect();
        assert_eq!(ids, ["1", "2", "3", "4"]);
    });
}
