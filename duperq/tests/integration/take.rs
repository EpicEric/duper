use smol::{LocalExecutor, io::AsyncReadExt};

use crate::common::{get_query_output_reader, parse_duper_values};

#[test]
fn take() {
    let query = r#"take 2 | format "${.id}""#;
    let values = parse_duper_values(&[
        r#"{id: "1"}"#,
        r#"{id: "2"}"#,
        r#"{id: "3"}"#,
        r#"{id: "4"}"#,
        r#"{id: "5"}"#,
    ]);
    let executor = LocalExecutor::new();
    let (mut reader, fut) = get_query_output_reader(&executor, query, values);
    smol::block_on(async {
        executor.run(fut).await;
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await.unwrap();
        let ids: Vec<&str> = buf.trim().lines().collect();
        assert_eq!(ids, ["1", "2"]);
    });
}
