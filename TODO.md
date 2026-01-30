Full Rust rewrite with rayon for parallelism
Or GPU acceleration if operations are parallelizable

Most likely best path: Rust rewrite with rayon for parallelism would give you 50-100x improvement over current ts-node + piscina setup. The iterations being independent makes this ideal for data parallelism.
