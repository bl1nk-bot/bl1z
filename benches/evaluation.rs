use bl1z::builtins;
use bl1z::{evaluate, parse, tokenize, Context, FunctionRegistry};
use criterion::{criterion_group, criterion_main, Criterion};

/// Benchmark basic arithmetic expressions
fn bench_basic_arithmetic(c: &mut Criterion) {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    let ctx = Context::new();

    c.bench_function("basic_arithmetic", |b| {
        b.iter(|| {
            let tokens = tokenize(std::hint::black_box("1 + 2 * 3")).unwrap();
            let ast = parse(&tokens).unwrap();
            let _result = evaluate(&ast, &ctx, &registry).unwrap();
        })
    });
}

/// Benchmark complex expressions with functions
fn bench_complex_expression(c: &mut Criterion) {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    let ctx = Context::new();

    c.bench_function("complex_expression", |b| {
        b.iter(|| {
            let tokens = tokenize(std::hint::black_box(
                "if(sum([1,2,3,4,5]) > 10, upper(\"hello\"), \"world\")",
            ))
            .unwrap();
            let ast = parse(&tokens).unwrap();
            let _result = evaluate(&ast, &ctx, &registry).unwrap();
        })
    });
}

/// Benchmark large array operations
fn bench_large_array(c: &mut Criterion) {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    let ctx = Context::new();

    // Create a large array sum expression
    let large_expr = format!(
        "sum([{}])",
        (1..=100)
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    c.bench_function("large_array_sum", |b| {
        b.iter(|| {
            let tokens = tokenize(std::hint::black_box(&large_expr)).unwrap();
            let ast = parse(&tokens).unwrap();
            let _result = evaluate(&ast, &ctx, &registry).unwrap();
        })
    });
}

/// Benchmark nested function calls
fn bench_nested_functions(c: &mut Criterion) {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    let ctx = Context::new();

    c.bench_function("nested_functions", |b| {
        b.iter(|| {
            let tokens = tokenize(std::hint::black_box(
                "upper(contains(\"HELLO WORLD\", lower(\"world\")))",
            ))
            .unwrap();
            let ast = parse(&tokens).unwrap();
            let _result = evaluate(&ast, &ctx, &registry).unwrap();
        })
    });
}

/// Benchmark date operations
fn bench_date_operations(c: &mut Criterion) {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    let ctx = Context::new();

    c.bench_function("date_operations", |b| {
        b.iter(|| {
            let tokens =
                tokenize(std::hint::black_box("year(date_add(\"2023-01-01\", 365))")).unwrap();
            let ast = parse(&tokens).unwrap();
            let _result = evaluate(&ast, &ctx, &registry).unwrap();
        })
    });
}

/// Benchmark map literal operations
fn bench_map_operations(c: &mut Criterion) {
    let mut registry = FunctionRegistry::new();
    builtins::register_all(&mut registry);
    let ctx = Context::new();

    c.bench_function("map_operations", |b| {
        b.iter(|| {
            let tokens = tokenize(std::hint::black_box("len([1, 2, 3])")).unwrap();
            let ast = parse(&tokens).unwrap();
            let _result = evaluate(&ast, &ctx, &registry).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_basic_arithmetic,
    bench_complex_expression,
    bench_large_array,
    bench_nested_functions,
    bench_date_operations,
    bench_map_operations
);
criterion_main!(benches);
