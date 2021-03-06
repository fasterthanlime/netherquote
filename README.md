# netherquote

Showcase for pathological compile times when using knuffel / chumsky / VERY
LARGE types.

## How to reproduce

The rust toolchain version is pinned to 1.59.0 stable already,
`.cargo/config.toml` defaults to lld but it probably doesn't make a big
difference here.

`cargo run` finishes in reasonable time, but `cargo run --release` spends
a _long_ time in "netherquote (bin)".

cargo timings don't show much, rustc self-profile shows a bunch of time spent in
thin-LTO, I don't know how to go much deeper.

```shell
$ summarize summarize netherquote-867123.mm_profdata | less
+-------------------------------------------------+-----------+-----------------+----------+------------+
| Item                                            | Self time | % of total time | Time     | Item count |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| LLVM_passes                                     | 12.66s    | 24.018          | 12.69s   | 1          |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| finish_ongoing_codegen                          | 11.80s    | 22.376          | 11.80s   | 1          |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| LLVM_module_optimize                            | 8.90s     | 16.879          | 8.90s    | 17         |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| LLVM_thin_lto_import                            | 4.89s     | 9.274           | 4.89s    | 16         |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| LLVM_module_codegen_emit_obj                    | 4.71s     | 8.942           | 4.71s    | 17         |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| LLVM_lto_optimize                               | 4.55s     | 8.629           | 4.55s    | 16         |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| codegen_module_perform_lto                      | 1.40s     | 2.658           | 15.67s   | 16         |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| codegen_module                                  | 1.12s     | 2.125           | 1.38s    | 16         |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| codegen_module_optimize                         | 1.01s     | 1.921           | 9.91s    | 17         |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| run_linker                                      | 232.54ms  | 0.441           | 232.54ms | 1          |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| codegen_fulfill_obligation                      | 209.15ms  | 0.397           | 345.49ms | 2308       |
+-------------------------------------------------+-----------+-----------------+----------+------------+
| normalize_projection_ty                         | 201.78ms  | 0.383           | 206.45ms | 823        |
+-------------------------------------------------+-----------+-----------------+----------+------------+
```

`cargo llvm-lines` show some multiple-pages-long types: these are generated by
one of [knuffel](https://crates.io/crates/knuffel)'s derive macro, which
uses [chumsky](http://crates.io/crates/chumsky) under the hood.

These generate a frightening amount of LLVM IR lines, considering `config.rs` is
barely over a hundred lines:

```shell
$ cargo llvm-lines | less
  Lines          Copies       Function name
  -----          ------       -------------
  322815 (100%)  7117 (100%)  (TOTAL)
   38985 (12.1%)  107 (1.5%)  <chumsky::combinator::Then<A,B> as chumsky::Parser<I,(O,U)>>::parse_inner
   21584 (6.7%)   331 (4.7%)  core::result::Result<T,E>::map
   21132 (6.5%)    36 (0.5%)  <chumsky::combinator::Or<A,B> as chumsky::Parser<I,O>>::parse_inner
   14220 (4.4%)   158 (2.2%)  <chumsky::combinator::Map<A,F,O> as chumsky::Parser<I,U>>::parse_inner
   14148 (4.4%)   108 (1.5%)  <chumsky::combinator::Or<A,B> as chumsky::Parser<I,O>>::parse_inner::zip_with
   12336 (3.8%)    48 (0.7%)  <chumsky::combinator::Repeated<A> as chumsky::Parser<I,alloc::vec::Vec<O>>>::parse_inner::{{closure}}
   10154 (3.1%)   158 (2.2%)  <chumsky::combinator::Map<A,F,O> as chumsky::Parser<I,U>>::parse_inner::{{closure}}
    6944 (2.2%)    14 (0.2%)  <chumsky::primitive::Choice<(X_,Y_,Z_),E> as chumsky::Parser<I,O>>::parse_inner
    6664 (2.1%)   180 (2.5%)  <chumsky::combinator::Or<A,B> as chumsky::Parser<I,O>>::parse_inner::{{closure}}
    5634 (1.7%)   112 (1.6%)  chumsky::stream::Stream<I,S>::attempt
    4680 (1.4%)    78 (1.1%)  chumsky::stream::Stream<I,S>::try_parse::{{closure}}
    4440 (1.4%)    24 (0.3%)  <chumsky::combinator::Repeated<A> as chumsky::Parser<I,alloc::vec::Vec<O>>>::parse_inner
    3807 (1.2%)   253 (3.6%)  <chumsky::debug::Silent as chumsky::debug::Debugger>::invoke
    3777 (1.2%)   251 (3.5%)  <chumsky::debug::Verbose as chumsky::debug::Debugger>::invoke
    3272 (1.0%)    14 (0.2%)  <chumsky::primitive::Filter<F,E> as chumsky::Parser<I,I>>::parse_inner
```

This might have more to do with codegen + LLVM and less with "making rustc data
structures / algorithms" more efficient, you be the judge!
