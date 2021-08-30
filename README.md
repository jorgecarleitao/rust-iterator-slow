# Bench Rust iterator problem

```bash
cargo bench --bench bench -- "2\^20"
```

on my computer

```
baseline 2^20           time:   [7.5919 ms 7.6029 ms 7.6224 ms]
get_bit1 2^20           time:   [5.9302 ms 5.9361 ms 5.9455 ms]
get_bit2 2^20           time:   [5.9447 ms 5.9716 ms 6.0057 ms]
```

i.e. placing the iterator's source code on its own crate yields a
20% degradation in performance, even when `inline` is used.
