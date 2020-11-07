## Note : Generate dependency graph

Must have GraphViz installed ( which provide `dot` binary ).

```
cargo +nightly install --git https://github.com/kbknapp/cargo-graph --force
```


Or add this to Cargo.lock with proper version
```
[root]
name="futurecommander"
version="0.x.x"
```

Then

```
cargo install cargo-graph
```

Generate `.dot` then `.png` files

```
cargo graph --optional-line-style dashed --optional-line-color red --optional-shape box --build-shape diamond --build-color green --build-line-color orange > doc/cargo-count.dot
dot -Tpng > doc/dependency-graph.png doc/cargo-count.dot
```
