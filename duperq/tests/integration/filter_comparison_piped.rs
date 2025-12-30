use smol::{LocalExecutor, io::AsyncReadExt};

use crate::common::{get_query_output_reader, parse_duper_values};

#[test]
fn filter_comparison_piped() {
    let query = r#"filter .http.statusCode < 400 | filter len(.http.history) > 2 | filter .http.duration < Duration('PT1S') | format "${.id:raw}""#;
    let values = parse_duper_values(&[
        include_str!("../data/1.duper"),
        r#"{id:"2",http:{statusCode:400,duration:Duration('PT0.5S'),history:[1,2,3]}}"#,
        r#"{id:"3",http:{statusCode:399,duration:Duration('PT1S'),history:[1,2,3]}}"#,
        r#"{id:"4",http:{statusCode:399,duration:Duration('PT0.5S'),history:[1,2]}}"#,
        r#"{id:"5",http:{duration:Duration('PT0.5S'),history:[1,2,3]}}"#,
        r#"{id:"6",http:{statusCode:399,history:[1,2,3]}}"#,
        r#"{id:"7",http:{statusCode:399,duration:Duration('PT0.5S')}}"#,
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
