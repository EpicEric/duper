use smol::{LocalExecutor, io::AsyncReadExt};

use crate::common::{get_query_output_reader, parse_duper_values};

#[test]
fn filter_type() {
    let query = r#"filter .http.address is Tuple | format "${.id:raw}""#;
    let values = parse_duper_values(&[
        include_str!("../data/1.duper"),
        r#"{id:"2",http:{}}"#,
        r#"{id:"3",http:{address:{}}}"#,
        r#"{id:"4",http:{address:()}}"#,
        r#"{id:"5",http:{address:[]}}"#,
        r#"{id:"6",http:{address:(null)}}"#,
    ]);
    let executor = LocalExecutor::new();
    let (mut reader, fut) = get_query_output_reader(&executor, query, values);
    smol::block_on(async {
        executor.run(fut).await;
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await.unwrap();
        let ids: Vec<&str> = buf.trim().lines().collect();
        assert_eq!(ids, ["1", "4", "6"]);
    });
}
