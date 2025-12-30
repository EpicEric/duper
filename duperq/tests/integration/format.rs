use smol::{LocalExecutor, io::AsyncReadExt};

use crate::common::{get_query_output_reader, parse_duper_values};

#[test]
fn filter_any() {
    let query = r#"filter exists(.) | format "${.id:raw},${.spanId},${cast(.timestamp, Instant)},${cast(.http.method, String):raw}""#;
    let values = parse_duper_values(&[
        include_str!("../data/1.duper"),
        "{}",
        r#"{id:2,spanId:Blah(false),timestamp:ZonedDateTime(null),http:{}}"#,
    ]);
    let executor = LocalExecutor::new();
    let (mut reader, fut) = get_query_output_reader(&executor, query, values);
    smol::block_on(async {
        executor.run(fut).await;
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await.unwrap();
        let formatted: Vec<&str> = buf.trim().lines().collect();
        assert_eq!(
            formatted,
            [
                r#"1,UUID("78ee3c78-c090-43f2-8568-f4542dc10ea5"),Instant('2025-11-29T22:21:45.133Z'),GET"#,
                r"<MISSING>,<MISSING>,<MISSING>,<MISSING>",
                r"2,Blah(false),<INVALID CAST>,<MISSING>",
            ]
        );
    });
}
