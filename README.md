# Simple File-System Database

Why another database?  
  
The problem with existing databases is that you have to learn them. 
Sfsdb is a high performance, incredibly simple yet extensible database, made to feel as native to the language as possible. If you know Rust, you can already use Sfsdb. 
Saving and Loading data is rather simple, so why is putting it in a database so complicated?  
  
Meet Sfsdb

## Features
 * No runtime dependencies or external configuration
 * High performance, Just run the benchmarks!
 * Optional caching, Automatically managed for a free performance boost at no usage cost
 * Optional indexing, Bundle index data together with your saves and [query them with the Rust language itself](https://github.com/AlmightyFloppyFish/sfsdb/blob/master/examples/indexed.rs#59)

## Status
Some concerns have been raised about how redundency is handled during concurrent access. This has made me rethink some of the design decision and some internal drastic changes will be made. For these reasons I cannot recommend using Sfsdb in production yet.  
  
## Usage
At it's core the only required functions are `new(location)`, `save(key, value)` and `<T>::load(key)`. However there's more abstractions using closures if you're using the Indexed version. The goal is to have a simple core that's extensible through intergrations of the Rust language itself.  
  
## Documentation
[API Documentation](https://docs.rs/sfsdb/)

Or, use [the examples](https://github.com/AlmightyFloppyFish/sfsdb/tree/master/examples).  
`$ git clone https://github.com/AlmightyFloppyFish/sfsdb; cd sfsdb`  
`$ cargo run --release --example simple`  
`$ cargo run --release --example cached`  
`$ cargo run --release --example indexed`  
`$ cargo run --release --example benchmark`  

## Benchmarks
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

## Cross-language support
There's an experimental [Golang version](https://github.com/AlmightyFloppyFish/sfsdb-go) and plans for a Haskell version.  
They might take some time however since they aren't just ports. But full rewrites made to feel as native to the languages as possible. 

## Contributing
Code contributions are absolutely welcomed! Just put in a pull-request and make sure you format with rustfmt
