use chumsky::Parser as _;
use duper::{DuperParser, DuperValue};
use piper::{Reader, pipe};
use smol::LocalExecutor;

use duperq::query;

pub(crate) fn parse_duper_values(values: &[&'static str]) -> Vec<DuperValue<'static>> {
    values
        .into_iter()
        .map(|input| DuperParser::parse_duper_trunk(input).unwrap())
        .collect()
}

pub(crate) fn get_query_output_reader(
    executor: &LocalExecutor<'_>,
    duperq_query: &str,
    values: Vec<DuperValue<'static>>,
) -> (Reader, impl Future<Output = ()>) {
    let (reader, writer) = pipe(32_768);
    let (pipeline_fns, output) = query().parse(duperq_query).unwrap();
    let mut tasks = Vec::new();
    let mut sink = pipeline_fns
        .into_iter()
        .rfold((output)(writer), |mut output, pipeline_fn| {
            let (sender, receiver) = smol::channel::bounded(128);
            tasks.push(executor.spawn(async move {
                while let Ok(value) = receiver.recv().await {
                    output.process(value).await;
                }
                output.close().await;
            }));
            (pipeline_fn)(sender)
        });
    tasks.push(executor.spawn(async move {
        for duper in values {
            sink.process(duper).await;
        }
    }));
    (reader, async move {
        futures::future::join_all(tasks).await;
    })
}
