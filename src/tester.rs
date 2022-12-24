use std::{
    path::Path,
    time::{Duration, Instant},
};

pub fn run_test(path: impl AsRef<Path>, solver: impl Fn(Vec<&str>) -> String) {
    run_test_lim(path, solver, u8::MAX);
}

pub fn run_silently(path: impl AsRef<Path>, solver: impl Fn(Vec<&str>) -> String) {
    run_test_with_path(
        path,
        solver,
        |n, t, expected, solved| {
            println!(
                "Test {n} is {} in {t:?}.",
                if expected == solved {
                    "passed"
                } else {
                    "FAILED"
                }
            );
            true
        },
        final_report,
    );
}

pub fn run_test_lim<Solver>(path: impl AsRef<Path>, solver: Solver, max_test: u8)
where
    Solver: Fn(Vec<&str>) -> String,
{
    run_test_with_path(
        path,
        solver,
        |n, t, expected, solved| {
            report_one(n, t, expected, solved);
            n < max_test
        },
        final_report,
    );
}

pub fn run_test_with_path(
    path: impl AsRef<Path>,
    solver: impl Fn(Vec<&str>) -> String,
    ok_to_proceed: impl Fn(u8, Duration, &str, &str) -> bool,
    report: impl FnOnce(i32, i32),
) {
    let mut test_number = 0_u8;
    let mut ok = 0;
    let mut failed = 0;
    while let Ok(str_n) =
        std::fs::read_to_string(path.as_ref().join(format!("test.{test_number}.in")))
    {
        println!(
            "Start test at path: {:?}",
            path.as_ref().join(format!("test.{test_number}.in"))
        );
        let input_data: Vec<&str> = str_n.lines().collect();
        let start = Instant::now();
        let solved = solver(input_data);
        let elapsed = Instant::now().duration_since(start);
        let result_path = path.as_ref().join(format!("test.{test_number}.out"));
        println!("Test finished. Compare with result at path {result_path:?}");
        let expected_result = std::fs::read_to_string(result_path).unwrap();
        let expected_result = expected_result.trim();
        if expected_result.trim() == solved {
            ok += 1;
        } else {
            failed += 1;
        }
        if ok_to_proceed(test_number, elapsed, expected_result, &solved) {
            test_number += 1;
        } else {
            break;
        }
    }
    report(ok, failed);
}

fn final_report(ok: i32, failed: i32) {
    let complete = ok + failed;
    println!("Have run {complete} tests.");
    println!("Success --- {ok}");
    if failed > 0 {
        println!("FAILED --- {failed}");
    }
}

fn report_one(n: u8, t: Duration, expected: &str, solved: &str) {
    let passed = expected == solved;
    println!(
        "Test {n} is {} in {t:?}.",
        if passed { "passed" } else { "FAILED" }
    );
    if !passed {
        println!(
            "Expected = '{}'\nSolved = {}",
            expected.chars().take(80).collect::<String>(),
            solved.chars().take(80).collect::<String>(),
        );
    }
}
