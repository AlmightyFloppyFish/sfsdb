# Simple File-System Database

## Features
 * No runtime dependencies or external configuration
 * High performance
	Just run the benchmarks!
 * Optional caching 
	Automatically managed caching for a free performance boost
 * Optional indexing
    Bundle index data together with your saves and [query them with the Rust language itself](https://github.com/AlmightyFloppyFish/sfsdb/blob/master/examples/indexed.rs#59)

## Status
The library is usable however won't be advertised until all checks under **Todo** are completed. 

### Todo
 * Improve caching performance (if possible)
 * Make indexes persist after restart (!important)
 * CachedIndexedDB
 * Documentation

 * Upload crate!

### Documentation
For now there's [the examples](https://github.com/AlmightyFloppyFish/sfsdb/tree/master/examples).
`$ git clone https://github.com/AlmightyFloppyFish/sfsdb; cd sfsdb`
`$ cargo run --release --example simple`
`$ cargo run --release --example cached`
`$ cargo run --release --example indexed`
`$ cargo run --release --example benchmark`
