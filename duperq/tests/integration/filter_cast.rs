use smol::{LocalExecutor, io::AsyncReadExt};

use crate::common::{get_query_output_reader, parse_duper_values};

#[test]
fn filter_range() {
    let query = r#"filter cast(.timestamp, Instant) > Instant('2025-11-01T00:00:00-03:00') | format "${.id}""#;
    let values = parse_duper_values(&[
        include_str!("../data/1.duper"),
        r#"{id:"2",timestamp:{}}"#,
        r#"{id:"3",timestamp:'2025-12-25'}"#,
        r#"{id:"4",timestamp:"invalid"}"#,
        r#"{id:"5",timestamp:Instant('2000-11-01T00:00:00-03:00')}"#,
        r#"{id:"6",timestamp:Instant('2026-11-01T00:00:00-03:00')}"#,
        r#"{id:"7",timestamp:ZonedDateTime('2026-11-01T00:00:00-03:00[America/Sao_Paulo]')}"#,
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
