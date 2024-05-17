# SS58 Vanity Prefix Generator

## Usage

After doing a `cargo build --release` with a Rust version at least 1.74.1 you can
run `target/release/vanity-prefix <FROM> <TO>` where both **FROM** and **TO** are
a number between 0 and 16383. It is quite fast to just run it as

```sh
$ target/release/vanity-prefix 0 16383 | grep ": hi"
address(11889): hi52wkaM6Mpff4qvpR2NvgVmFYLivWZCSEMHjkBiDWpVbq7r3
address(12145): hiApc1ZtVk7YPDuLHKZFx4Q2gBpsDZjpxRHqsMsMWyg667QG3
address(12401): hiGcGGZRu8QR7NxjkE68ySJJ6qK1WcvTUcEPzyYzpSXgaPbP8
address(12657): hiNPvXYyJWhHqY29D8d1zpCZXUo9og75zoAx8bEe7uPH4g3vR
address(12913): hiUBanYWhtzAZh5Yg39u2C6px8HJ6jHiWz7WGCvHRNEsYxSdJ
address(13169): hiZyF3Y47HH3Hr8x8wgn3a16NmmSPnUM3B44Ppbviq6U3Enwk
address(13425): hifkuJXbWfZv21CMbrDf4wuMoRFagqeyZMzcXSHa2Hx4XWpiZ
address(13681): himYZZX8v3rnkAFm4kkY6KodE4jiytqc5YwAf3yDKkof1oExZ
address(13937): hisLDpWgKS9fUKKAXfHR7hhteiDsGx2EbjsinferdDfFW5i1J
address(14193): hiy7t5WDipSYCUNZzZpJ95cA5Mi1a1Cs7vpGvHLVvgWqzMonE
```

to look for prefixes that generate addresses starting with "hi". Note that some
characters are missing from the Base58 BTC encoding, so not all prefixes are
possible to reach. Also, some prefixes that are composed of valid characters of
the Base58 BTC alphabet below are unreachable because of how maths works.ű

## The Base58 BTC alphabet

To avoid easy to misread characters, some are left out from the lower-, upper-case
English alphabetic characters and the arabic numbers:

`123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`

Note that Monero and BTC uses this alphabet, but there exists a different one for
Ripple and Flickr, too.
