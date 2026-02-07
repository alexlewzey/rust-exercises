fn is_id_invalid(id: i64) -> Option<i64> {
    let id_string = id.to_string();
    let id_len = id_string.chars().count();
    for chunk_size in 1..(id_len / 2) + 1 {
        if id_len % chunk_size == 0 {
            let first: String = id_string.chars().take(chunk_size).collect();
            let mut all_eq = true;
            for idx in (0..id_len).step_by(chunk_size) {
                let chunk: String = id_string.chars().skip(idx).take(chunk_size).collect();
                if chunk != first {
                    all_eq = false;
                    break;
                }
            }
            if all_eq {
                return Some(id);
            }
        }
    }
    None
}

fn main() {
    let data = "3737332285-3737422568,5858547751-5858626020,166911-236630,15329757-15423690,753995-801224,1-20,2180484-2259220,24-47,73630108-73867501,4052222-4199117,9226851880-9226945212,7337-24735,555454-591466,7777695646-7777817695,1070-2489,81504542-81618752,2584-6199,8857860-8922218,979959461-980003045,49-128,109907-161935,53514821-53703445,362278-509285,151-286,625491-681593,7715704912-7715863357,29210-60779,3287787-3395869,501-921,979760-1021259";

    let mut invalid_id_sum: i64 = 0;
    let rngs = data.split(",");
    for rng in rngs {
        let (rng_start, rng_end) = rng.split_once("-").unwrap();
        let rng_start: i64 = rng_start.parse().unwrap();
        let rng_end: i64 = rng_end.parse().unwrap();
        for id in rng_start..rng_end + 1 {
            let result = is_id_invalid(id);
            if let Some(id) = result {
                invalid_id_sum += id
            }
        }
    }
    println!("invalid_id_sum: {invalid_id_sum}")
}

#[test]
fn test_is_id_invalid() {
    let id: i64 = 2121212121;
    let res = is_id_invalid(id);
    assert_eq!(res, Some(2121212121));

    let id: i64 = 123123;
    let res = is_id_invalid(id);
    assert_eq!(res, Some(123123));

    let id: i64 = 11;
    let res = is_id_invalid(id);
    assert_eq!(res, Some(11));

    let id: i64 = 1234;
    let res = is_id_invalid(id);
    assert_eq!(res, None);
}
