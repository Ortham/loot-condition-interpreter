#[macro_use]
extern crate criterion;
extern crate loot_condition_interpreter;

use criterion::Criterion;
use loot_condition_interpreter::{Expression, GameType, State};

fn generate_active_plugins() -> Vec<String> {
    let mut vec: Vec<String> = (0..255).map(|i| format!("Blank{}.esm", i)).collect();
    vec.push("Blank.esm".into());
    vec
}

fn generate_plugin_versions() -> Vec<(String, String)> {
    let mut vec: Vec<(String, String)> = (0..255)
        .map(|i| (format!("Blank{}.esm", i), "5".to_string()))
        .collect();
    vec.push(("Blank.esm".into(), "5".into()));
    vec
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Expression.eval() file(path)", |b| {
        let state = State::new(GameType::Tes4, ".".into(), ".".into());
        let expression = Expression::parse("file(\"Cargo.toml\")").unwrap().1;

        b.iter(|| {
            assert!(expression.eval(&state).unwrap());
        });
    });

    c.bench_function("Expression.eval() file(regex)", |b| {
        let state = State::new(GameType::Tes4, ".".into(), ".".into());
        let expression = Expression::parse("file(\"Cargo.*\")").unwrap().1;

        b.iter(|| {
            assert!(expression.eval(&state).unwrap());
        });
    });

    c.bench_function("Expression.eval() active(path)", |b| {
        let state = State::new(
            GameType::Tes4,
            "testing-plugins/Oblivion/Data".into(),
            ".".into(),
        ).with_active_plugins(&generate_active_plugins());

        let expression = Expression::parse("active(\"Blank.esm\")").unwrap().1;

        b.iter(|| {
            assert!(expression.eval(&state).unwrap());
        });
    });

    c.bench_function("Expression.eval() active(regex)", |b| {
        let state = State::new(
            GameType::Tes4,
            "testing-plugins/Oblivion/Data".into(),
            ".".into(),
        ).with_active_plugins(&generate_active_plugins());

        let expression = Expression::parse("active(\"Blank.*\")").unwrap().1;

        b.iter(|| {
            assert!(expression.eval(&state).unwrap());
        });
    });

    c.bench_function("Expression.eval() many()", |b| {
        let state = State::new(GameType::Tes4, ".".into(), ".".into());
        let expression = Expression::parse("many(\"Cargo.*\")").unwrap().1;

        b.iter(|| {
            assert!(expression.eval(&state).unwrap());
        });
    });

    c.bench_function("Expression.eval() many_active()", |b| {
        let state = State::new(
            GameType::Tes4,
            "testing-plugins/Oblivion/Data".into(),
            ".".into(),
        ).with_active_plugins(&generate_active_plugins());

        let expression = Expression::parse("many_active(\"Blank.*\")").unwrap().1;

        b.iter(|| {
            assert!(expression.eval(&state).unwrap());
        });
    });

    c.bench_function("Expression.eval() checksum()", |b| {
        let state = State::new(
            GameType::Tes4,
            "testing-plugins/Oblivion/Data".into(),
            ".".into(),
        );
        let expression = Expression::parse("checksum(\"Blank.esm\", 374E2A6F)")
            .unwrap()
            .1;

        b.iter(|| {
            assert!(expression.eval(&state).unwrap());
        });
    });

    c.bench_function("Expression.eval() version(plugin)", |b| {
        let state = State::new(
            GameType::Tes4,
            "testing-plugins/Oblivion/Data".into(),
            ".".into(),
        ).with_plugin_versions(&generate_plugin_versions());

        let expression = Expression::parse("version(\"Blank.esm\", \"5.0\", ==)")
            .unwrap()
            .1;

        b.iter(|| {
            assert!(expression.eval(&state).unwrap());
        });
    });

    c.bench_function("Expression.eval() version(executable)", |b| {
        let state = State::new(GameType::Tes4, ".".into(), ".".into());
        let expression = Expression::parse(
            "version(\"loot_api-0.13.8-0-g47797cc_dev-win32/loot_api.dll\", \"0.13.8.0\", ==)",
        ).unwrap()
        .1;

        b.iter(|| {
            assert!(expression.eval(&state).unwrap());
        });
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);