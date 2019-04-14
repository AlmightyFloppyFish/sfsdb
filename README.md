# Simple File-System Database

## Features
 * No runtime dependencies or external configuration
 * High performance, Just run the benchmarks!
 * Optional caching, Automatically managed for a free performance boost at no usage cost
 * Optional indexing, Bundle index data together with your saves and [query them with the Rust language itself](https://github.com/AlmightyFloppyFish/sfsdb/blob/master/examples/indexed.rs#59)

## Status
The library is usable however won't be advertised until all checks under **Todo** are completed. 

### Todo
 - [x] Indexing
 - [x] Write Examples and Benchmarks
 - [ ] Improve caching performance (if possible)
 - [x] Make indexes persist after restart (!important)
 - [x] CachedIndexedDB
 - [ ] Documentation
 - [ ] Upload crate!

### Documentation
For now there's [the examples](https://github.com/AlmightyFloppyFish/sfsdb/tree/master/examples).  
`$ git clone https://github.com/AlmightyFloppyFish/sfsdb; cd sfsdb`  
`$ cargo run --release --example simple`  
`$ cargo run --release --example cached`  
`$ cargo run --release --example indexed`  
`$ cargo run --release --example benchmark`  

### Benchmarks
Don't want to compile the benchmark example? Well here's my results
```
(Simple) Saving justin 1000 times took: 40.561932ms
(Simple) Loading justin (with key '400') 1000 times took: 3.490118ms

(Cached) Saving justin 1000 times took: 44.653967ms
(Cached) Loading justin (with key '400') 1000 times took: 776.095µs

(Indexed + Cached) Saving justin 1000 times took: 85.815535ms
(Indexed + Cached) Loading justin (with key '400') 1000 times took: 662.987µs
(Indexed + Cached) Querying for all logged-in users (which yielded 500 results) took: 44.308µs
(Indexed + Cached) Querying for all locked-out users (which yielded 179 results) took: 24.893µs
```
```
Intel i7-6600U (4) @ 3.400GHz
SATA SSD
16GB RAM
```
