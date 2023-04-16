use lazy_regex::regex_captures;
use log::debug;

use crate::{
    request::{list, queries},
    ResultDyn,
};

pub fn stats(host: &str) -> ResultDyn<Stats> {
    let (mut limit, mut start) = (Some(30), Some(1));
    let mut ns_query = Vec::new();
    loop {
        let list_response = list(host, limit, start)?;
        for run_id in list_response.run_ids {
            let count = count_queries(host, &run_id)?;
            debug!("{run_id}: {count} queries.");
            ns_query.push(count);
        }
        match list_response.next {
            Some(next) => {
                (limit, start) = limit_start(&next)?;
                debug!("Next list, limit: {limit:?}, start: {start:?}.");
            }
            None => break,
        }
    }

    debug!("Number of queries: {ns_query:#?}");
    let (mean, variance) = mean_variance(&ns_query);

    Ok(Stats { mean, variance })
}

fn mean_variance(data: &[usize]) -> (f64, f64) {
    let length = data.len() as f64;
    let sum = data.iter().sum::<usize>() as f64;
    let mean = sum / length;

    let variance = data
        .iter()
        .map(|value| (*value as f64 - mean).powi(2))
        .sum::<f64>()
        / length;

    (mean, variance)
}

fn count_queries(host: &str, run_id: &str) -> ResultDyn<usize> {
    let mut count = 0;
    let (mut limit, mut start) = (Some(30), Some(1));
    loop {
        let quries = queries(host, run_id, limit, start)?;
        count += quries.queries.len();
        match quries.next {
            Some(next) => {
                (limit, start) = limit_start(&next)?;
                debug!("Next queries, limit: {limit:?}, start: {start:?}.");
            }
            None => break,
        }
    }

    Ok(count)
}

fn limit_start(path: &str) -> ResultDyn<(Option<usize>, Option<usize>)> {
    let limit = match regex_captures!(r#"limit=(\d*)"#, path) {
        Some((_, limit)) => Some(limit.parse()?),
        None => None,
    };
    let start = match regex_captures!(r#"start=(\d*)"#, path) {
        Some((_, start)) => Some(start.parse()?),
        None => None,
    };
    Ok((limit, start))
}

#[derive(Debug)]
pub struct Stats {
    pub mean: f64,
    pub variance: f64,
}
