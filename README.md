# Latency - Code Examples

<img src=".github/assets/hero.jpg" alt="Latency book cover" width="400">

This repository contains code examples and practical implementations from the book **"Latency"** by Pekka Enberg, published by Manning Publications.

## About the Book

[**Latency**](https://www.manning.com/books/latency) is a comprehensive guide to understanding, measuring, and optimizing latency in software systems. The book covers practical techniques for delivering low-latency software, from spotting latency issues to implementing solutions that keep your applications responsive.

### What You'll Learn

- What latency is—and what it is not
- How to model and measure latency
- Organizing your application data for low latency
- Making your code run faster
- Hiding latency when you can't reduce it

## Building and Running the Examples

The examples are organized by chapter. Each chapter directory contains a `rust/` subdirectory with one or more standalone Cargo crates, except chapter 2, which contains Python scripts. There is no top-level workspace: build and run each example from its own directory.

### Prerequisites

- **Rust** via [rustup](https://rustup.rs/). The chapter 3, 5, and 6 examples pin Rust 1.65.0 with a `rust-toolchain` file, which rustup installs automatically. The other examples use whatever stable toolchain you have installed.
- **Python 3** for chapter 2, with `ping3`, `pandas`, `numpy`, `matplotlib`, `statsmodels`, and `hdrh` installed.
- **PostgreSQL** and OpenSSL development headers for the PostgreSQL examples in chapters 3 and 6.
- **Node.js** and [Wrangler](https://developers.cloudflare.com/workers/wrangler/) for the Cloudflare Workers example in chapter 4.

### Building a Rust example

```bash
cd chapter-07/rust/fibonacci
cargo build --release
```

Examples that ship a Criterion benchmark (chapters 7, 8, and 9) are run with:

```bash
cargo bench
```

Criterion writes HTML reports under `target/criterion/`.

### Chapter 2: Measuring latency

Measure ping latency to a host and plot the results:

```bash
cd chapter-02
pip install ping3 pandas numpy matplotlib statsmodels hdrh
sudo ./ping.py google.com 1000     # writes ping-google.com.csv
./plot-hist.py
./plot-ecdf.py
./plot-hdrhist.py
```

The plot scripts read `ping-google.com.csv`, so either ping `google.com` or edit the filename in the scripts. Sending ICMP pings typically requires root.

### Chapter 3: Hello SQLite and PostgreSQL

Both servers listen on `127.0.0.1:8080` and read the database location from `DATABASE_URL`:

```bash
cd chapter-03/rust/hello-sqlite
DATABASE_URL=hello.db cargo run --release
```

```bash
cd chapter-03/rust/hello-postgresql
DATABASE_URL=postgres://user:password@localhost/dbname cargo run --release
```

Then measure request latency from another terminal:

```bash
cd chapter-03/rust/http-bench
cargo run --release
```

### Chapter 4: Replication

The key-value replication example has two binaries. Start the primary, which listens on port 8080, then start one or more replicas on other ports and join them to the primary:

```bash
cd chapter-04/rust/replication-kv
cargo run --bin primary            # in one terminal
cargo run --bin replica 8081       # in another terminal, then type: JOIN 127.0.0.1:8080
```

The Cloudflare Workers example needs a libSQL database URL and token stored as Wrangler secrets:

```bash
cd chapter-04/rust/workers-libsql
npm install
wrangler secret put LIBSQL_CLIENT_URL
wrangler secret put LIBSQL_CLIENT_TOKEN
npm run dev                        # local development
npm run deploy                     # deploy to Cloudflare
```

### Chapter 5: Multiple server nodes

The SQLite server takes a node ID and listens on port `8080 + id`. Start several nodes, then run the benchmark against all of them:

```bash
cd chapter-05/rust/hello-sqlite
DATABASE_URL=hello.db cargo run --release -- --id 0
DATABASE_URL=hello.db cargo run --release -- --id 1
```

```bash
cd chapter-05/rust/http-bench
cargo run --release -- --node-count 2
```

### Chapter 6: Caching

Same setup as the chapter 3 PostgreSQL example, with an in-process cache in front of the database:

```bash
cd chapter-06/rust/hello-postgresql-cached
DATABASE_URL=postgres://user:password@localhost/dbname cargo run --release
```

### Chapters 7, 8, and 9: Benchmarks

These crates exist mainly for their Criterion benchmarks:

```bash
cd chapter-07/rust/fibonacci && cargo bench
cd chapter-08/rust/locking-bench && cargo bench
cd chapter-08/rust/spsc-queue && cargo bench
cd chapter-09/rust/sequential-vs-concurrent && cargo bench
```

The chapter 7 example also has a `benchmark_pprof` bench that produces flame graphs via `pprof`.

## Contributing

This repository contains the official code examples from the book. If you find issues or have suggestions for improvements:

1. Check existing issues first
2. Create a detailed issue report
3. For code improvements, please reference the specific book chapter

## Additional Resources

- [Book on Manning's website](https://www.manning.com/books/latency)
- [Book on Amazon](https://amzn.to/4nKI3Un)

## License

MIT

## Support

For questions about the book content, please refer to Manning's support channels. For code-specific issues, please use this repository's issue tracker.

---

*This code repository accompanies "Latency" by Pekka Enberg. The book provides comprehensive explanations and theory behind these practical implementations.*
