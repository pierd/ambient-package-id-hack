# ambient-package-id-hack

Generate valid [Ambient](https://ambient.run) package IDs with a given prefix.

## Installation

```sh
$ cargo install ambient-package-id-hack
```

## Usage

```sh
$ ambient-package-id-hack fancygame    
fancygame: fancygamecxzpnhltyojv6cy5wlzfnaj
$ ambient-package-id-hack mysuperawesomepackageid
mysuperawesomepackageid: mysuperawesomepackacps4fgbzwwi7l
$ ambient-package-id-hack fancygame mysuperawesomepackageid
fancygame: fancygamecvxdrhuhw234i2fbku3eoi7
mysuperawesomepackageid: mysuperawesomepackacps4fgbzwwi7l
```

Note that if the prefix is too long then it will be truncated.
