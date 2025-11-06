# CCHoL

Note: **NOT IN "USEFUL" STAGE YET** - everything is in heavy flux with things added, removed, moved, refactored, redesigned, ad nauseum…

CCHoL is a project that aims to be a "Central Casting, Heroes of Legend"-based RPG character background generator.

## Testing Stuff

When messing with JSONs, it's a good idea to now and then run:

```bash
cargo test data_integrity
```

## OS Agnostic?

Yea, **CCHoL** is pretty much OS-agnostic. File access of JSONs might choke on some weirdo system that doesn't obey
e.g. `./data/somefile.json` path format. No such system comes into mind right now, however…
