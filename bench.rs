#[macro_use]

use std::{collections::HashMap, fs::File, io::Read, path::Path};

use complete_json::{json_value, Value};

use {
    combine::{
        error::{Commit, ParseError},
        stream::{
            buffered,
            position::{self, SourcePosition},
            IteratorStream,
        },
        EasyParser, Parser, Stream, StreamOnce,
    },
    criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion},
};


fn test_data() -> String {
    let mut data = String::new();
    File::open(&Path::new(&"benches/data.json"))
        .and_then(|mut file| file.read_to_string(&mut data))
        .unwrap();
    data
}

fn bench_json(bencher: &mut Bencher<'_>) {
    let data = test_data();
    let mut parser = json_value();
    match parser.easy_parse(position::Stream::new(&data[..])) {
        Ok((Value::Array(_), _)) => (),
        Ok(_) => panic!(),
        Err(err) => {
            println!("{}", err);
            panic!();
        }
    }
    bencher.iter(|| {
        let result = parser.easy_parse(position::Stream::new(&data[..]));
        black_box(result)
    });
}

fn bench_json_core_error(bencher: &mut Bencher<'_>) {
    let data = test_data();
    let mut parser = json_value();
    match parser.parse(position::Stream::new(&data[..])) {
        Ok((Value::Array(_), _)) => (),
        Ok(_) => panic!(),
        Err(err) => {
            println!("{}", err);
            panic!();
        }
    }
    bencher.iter(|| {
        let result = parser.parse(position::Stream::new(&data[..]));
        black_box(result)
    });
}

fn bench_json_core_error_no_position(bencher: &mut Bencher<'_>) {
    let data = test_data();
    let mut parser = json_value();
    match parser.parse(&data[..]) {
        Ok((Value::Array(_), _)) => (),
        Ok(_) => panic!(),
        Err(err) => {
            println!("{}", err);
            panic!();
        }
    }
    bencher.iter(|| {
        let result = parser.parse(&data[..]);
        black_box(result)
    });
}

fn bench_buffered_json(bencher: &mut Bencher<'_>) {
    let data = test_data();
    bencher.iter(|| {
        let buffer =
            buffered::Stream::new(position::Stream::new(IteratorStream::new(data.chars())), 1);
        let mut parser = json_value();
        match parser.easy_parse(position::Stream::with_positioner(
            buffer,
            SourcePosition::default(),
        )) {
            Ok((Value::Array(v), _)) => {
                black_box(v);
            }
            Ok(_) => panic!(),
            Err(err) => {
                println!("{}", err);
                panic!();
            }
        }
    });
}

fn bench(c: &mut Criterion) {
    c.bench_function("json", bench_json);
    c.bench_function("json_core_error", bench_json_core_error);
    c.bench_function(
        "json_core_error_no_position",
        bench_json_core_error_no_position,
    );
    c.bench_function("buffered_json", bench_buffered_json);
}

criterion_group!(json, bench);
criterion_main!(json);
