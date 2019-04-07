Garder la hiérarchie ou pas ? Descendante ou montante ?
Path ou PathBuf ?
Un BTree avec les NodeIndex des dossiers en clé ?

Parent NodeIndex -> Child NodeIndex




## Virtual versionning file systems

https://github.com/git-lfs/git-lfs/blob/master/docs/spec.md

https://code.fb.com/core-data/scaling-mercurial-at-facebook/

> Notify changes ( e.g hgwatchman ) ? distributed cache of content ( e.g remotefilelog ) ?


## Lowcost graph based data representations in Rust

### Graph Indices

https://docs.rs/petgraph/0.4.13/petgraph/
http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

### Vector Indices

https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
https://github.com/saschagrunert/indextree


## Programs written in rust which handle fs aspects

https://github.com/kamiyaa/joshuto
https://github.com/sharkdp/fd


## Rust Language

https://rust-lang-nursery.github.io/edition-guide/rust-2018/index.html
https://doc.rust-lang.org/nightly/edition-guide/rust-2018
https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
https://doc.rust-lang.org/reference/index.html
https://doc.rust-lang.org/rust-by-example/index.html
https://github.com/mre/idiomatic-rust


## Shells

https://www.joshmcguigan.com/blog/build-your-own-shell-rust/
https://doc.rust-lang.org/std/process/struct.Command.html


## Iterators

https://burgers.io/wrapped-iterators-in-rust
https://burgers.io/extending-iterator-trait-in-rust
https://medium.com/@jordan_98525/reference-iterators-in-rust-5603a51b5192
https://doc.rust-lang.org/stable/std/iter/
https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html
https://stackoverflow.com/questions/32682876/is-there-any-way-to-return-a-reference-to-a-variable-created-in-a-function

> extending an iterator !! https://beyermatthias.de/blog/2018/02/02/extending-an-iterator-in-rust/

## Referentiation

https://bryce.fisher-fleig.org/blog/strategies-for-returning-references-in-rust/index.html
https://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/

## TUI / GUI

https://crates.io/crates/cursive
https://github.com/maps4print/azul

## OO Pattern

State Pattern => https://doc.rust-lang.org/1.30.0/book/second-edition/ch17-03-oo-design-patterns.html
Adapter Pattern => https://github.com/jdavis/rust-design-patterns/blob/master/patterns/adapter.rs

## Bitbucket pipelining

build artifacts : https://confluence.atlassian.com/bitbucket/deploy-build-artifacts-to-bitbucket-downloads-872124574.html
pipeline yml format : https://confluence.atlassian.com/bitbucket/configure-bitbucket-pipelines-yml-792298910.html

## Generics ( Sized Monomorphisation ) against traits ( Unsized )

https://stackoverflow.com/questions/24635146/can-this-code-be-written-without-generics?rq=1

## crates

CLI testing :  https://crates.io/crates/assert_cmd

## Rendering ( & gaming )

http://arewegameyet.com/

## Threading

https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa

## Rust and nodejs

https://neon-bindings.com/
https://nwjs.io/
https://proton-native.js.org/#/

## Rust lints

https://github.com/rust-lang/rust-clippy

## QA

https://github.com/rust-lang/miri#bugs-found-by-miri

## Functional programming

https://www.amazon.com/Hands-Functional-Programming-Rust-applications/dp/1788839358/ref=as_li_ss_tl?ie=UTF8&linkCode=sl1&tag=tutorialedge-20&linkId=3664a43d9e5e1a441704bdce071983df
