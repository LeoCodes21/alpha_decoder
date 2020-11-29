# alpha_decoder

`alpha_decoder` is a Rust command-line tool for decoding Alpha2-based shellcode.

## Roadmap

This tool is currently a work-in-progress. Here is a rough to-do list:

- [ ] Implement `mixedcase_ascii_nocompress` format
- [ ] Implement `uppercase_ascii_nocompress` format
- [ ] Implement `uppercase_unicode` format
- [ ] Implement `mixedcase_unicode_nocompress` format
- [ ] Implement `uppercase_unicode_nocompress` format
- [ ] Create a proper API for identifying and parsing Alpha2 formats
- [ ] Expose more information about Alpha2 payloads to the user
- [ ] Expose functionality as a crate
- [ ] Add tests

## Installation
Download the latest build from [repository releases](https://github.com/LeoCodes21/alpha_decoder/releases).

## Usage

```
# Decode shellcode and write result as a hex string to standard output
> alpha_decoder.exe PYIIIIIIIIIIQZVTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJINXEQEPEP30YSA
Decoded shellcode as Uppercase ASCII: b801000000c3

# Decode shellcode and print a formatted hex dump
> alpha_decoder.exe PYIIIIIIIIIIQZVTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJINXEQEPEP30YSA -h
Decoded shellcode as Uppercase ASCII: b801000000c3
Length: 6 (0x6) bytes
0000:   b8 01 00 00  00 c3                                   ......

# Decode shellcode and write result to a file
> alpha_decoder.exe PYIIIIIIIIIIQZVTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJINXEQEPEP30YSA -o shellcode_out.bin
[shellcode_out.bin contains the result of decoding]
```

## Contributing
Pull requests are welcome and greatly appreciated. Before making any major changes, please consider opening an issue for discussion.

## License
This project is released under the [MIT](https://choosealicense.com/licenses/mit/) license.