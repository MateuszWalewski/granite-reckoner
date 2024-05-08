use criterion::{criterion_group, criterion_main, Criterion};
use granite_reckoner::Column;

mod common;

fn bench_column_sum_1_node(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("sum_t", |b| b.iter(|| column.sum_t(1)));

    group.finish();
}

fn bench_column_sum_2_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");
    group.bench_function("sum_t", |b| b.iter(|| column.sum_t(2)));

    group.finish();
}

fn bench_column_sum_4_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");
    group.bench_function("sum_t", |b| b.iter(|| column.sum_t(4)));

    group.finish();
}

fn bench_column_sum_x2_1_node(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("sum_x2_t", |b| b.iter(|| column.sum_x2_t(1)));

    group.finish();
}

fn bench_column_sum_x2_2_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");
    group.bench_function("sum_x2_t", |b| b.iter(|| column.sum_x2_t(2)));

    group.finish();
}

fn bench_column_sum_x2_4_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");
    group.bench_function("sum_x2_t", |b| b.iter(|| column.sum_x2_t(4)));

    group.finish();
}

fn bench_column_min_1_node(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("min_t", |b| b.iter(|| column.min_t(1)));

    group.finish();
}

fn bench_column_min_2_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("min_t", |b| b.iter(|| column.min_t(2)));

    group.finish();
}

fn bench_column_min_4_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("min_t", |b| b.iter(|| column.min_t(4)));

    group.finish();
}

fn bench_column_max_1_node(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("max_t", |b| b.iter(|| column.max_t(1)));

    group.finish();
}

fn bench_column_max_2_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("max_t", |b| b.iter(|| column.max_t(2)));

    group.finish();
}

fn bench_column_max_4_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("max_t", |b| b.iter(|| column.max_t(4)));

    group.finish();
}

fn bench_column_variance_1_node(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("variance_t", |b| b.iter(|| column.variance_t(1)));

    group.finish();
}

fn bench_column_variance_2_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("variance_t", |b| b.iter(|| column.variance_t(2)));

    group.finish();
}

fn bench_column_variance_4_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("variance_t", |b| b.iter(|| column.variance_t(4)));

    group.finish();
}

fn bench_column_stddev_1_node(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("stddev_t", |b| b.iter(|| column.stddev_t(1)));

    group.finish();
}

fn bench_column_stddev_2_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("stddev_t", |b| b.iter(|| column.stddev_t(2)));

    group.finish();
}

fn bench_column_stddev_4_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("stddev_t", |b| b.iter(|| column.stddev_t(4)));

    group.finish();
}

fn bench_column_moment_i_1_node(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("moment_i_t", |b| b.iter(|| column.moment_i_t(1)));

    group.finish();
}

fn bench_column_moment_i_2_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("moment_i_t", |b| b.iter(|| column.moment_i_t(2)));

    group.finish();
}

fn bench_column_moment_i_4_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("moment_i_t", |b| b.iter(|| column.moment_i_t(4)));

    group.finish();
}

fn bench_column_moment_ii_1_node(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("moment_ii_t", |b| b.iter(|| column.moment_ii_t(1)));

    group.finish();
}

fn bench_column_moment_ii_2_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("moment_ii_t", |b| b.iter(|| column.moment_ii_t(2)));

    group.finish();
}

fn bench_column_moment_ii_4_nodes(c: &mut Criterion) {
    let container: Vec<f64> = common::load_data("data_3M.csv");
    let column = Column::new();
    let column = column.add_data(container);
    let mut group = c.benchmark_group("column_sum");

    group.bench_function("moment_ii_t", |b| b.iter(|| column.moment_ii_t(4)));

    group.finish();
}

criterion_group!(
    benches,
    bench_column_sum_1_node,
    bench_column_sum_2_nodes,
    bench_column_sum_4_nodes,
    bench_column_sum_x2_1_node,
    bench_column_sum_x2_2_nodes,
    bench_column_sum_x2_4_nodes,
    bench_column_variance_1_node,
    bench_column_variance_2_nodes,
    bench_column_variance_4_nodes,
    bench_column_stddev_1_node,
    bench_column_stddev_2_nodes,
    bench_column_stddev_4_nodes,
    bench_column_moment_i_1_node,
    bench_column_moment_i_2_nodes,
    bench_column_moment_i_4_nodes,
    bench_column_moment_ii_1_node,
    bench_column_moment_ii_2_nodes,
    bench_column_moment_ii_4_nodes,
    bench_column_min_1_node,
    bench_column_min_2_nodes,
    bench_column_min_4_nodes,
    bench_column_max_1_node,
    bench_column_max_2_nodes,
    bench_column_max_4_nodes,
);
criterion_main!(benches);
