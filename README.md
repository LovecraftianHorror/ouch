# Ouch!

### This readme is a WIP draft, and does not represent the reality
See the old README at https://github.com/vrmiguel/ouch/tree/0f453e9dfc70066056b9cc40e8032dcc6ee703bc
<!-- ![ouch_image](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcR5ilNDTFZZ-Vy_ctm2YyAe8Yk0UT7lB2hIhg&usqp=CAU)  -->

`ouch` loosely stands for Obvious Unified Compression (ᵃⁿᵈ ᵈᵉᶜᵒᵐᵖʳᵉˢˢᶦᵒⁿ) Helper and aims to be an easy and intuitive way of compressing and decompressing files on the command-line.

- [Usage](#Usage)
    - [Decompress](#Decompress)
    - [Compress](#Compress)
    - [Convert](#Convert)
- [Supported Formats](#Supported-formats)
- [Installation](#Installation)
- [Supported operating systems](#Supported-operating-systems)


## Usage

`ouch` uses the file extensions to infer the file formats.

For example, `ouch compress a b c.zip` tells to compress `a` and `b` into the same `c.zip` compressed file.

### Decompressing files
<!-- botei caps e mais um #, acho que ficou bom para os comandos -->

To decompress, just call `ouch` passing the input files.

Use the `-o, --output` flag to redirect the output to a folder.

```bash
# Decompress `a.zip`
ouch a.zip

# Decompress multiple files
ouch a.zip b.tar.gz

# Decompress multiple files, but inside new_folder
ouch a.zip b.tar.gz -o new_folder
```

### Compressing files and directories

The `compress` subcommand accepts files and folders where the **last** one is the desired **output file**.

```bash
# Compress four files into `archive.zip`
ouch compress a b c d archive.zip

# Compress three files into `.tar.bz2` archive
ouch compress a.mp4 b.jpg c.png files.tar.bz2

# Compress a folder and a file into `videos.tar.xz`
ouch compress Videos/ funny_meme.mp4 videos.tar.xz
```

### Converting between compressed archives

```
# Will decompress a.tar.gz b.tar.bz and compress it into new-archive.tar.xz2
ouch convert a.tar.gz b.tar.bz new-archive.tar.xz2
```

### Listing the elements of an archive

(TODO -- not implemented at all)

```
# Shows the files contained in videos.tar.xz
ouch list videos.tar.xz
1. .....
```

## Supported formats


|               | .tar | .zip | .tar.\*¹ | .zip.\*² | .bz, .bz2 | .gz | .xz, .lz, .lzma | .7z |
|:-------------:|:----:|:----:|:--------:|:--------:|:---------:| --- |:---------------:| --- |
| Decompression |  ✓   |  ✓   |    ✓     |    ✓     |     ✓     | ✓   |        ✓        | ✗   |
|  Compression  |  ✓   |  ✓   |    ✓     |    ✓     |     ✓     | ✓   |        ✓        | ✗   |

```
Note: .tar.*¹: .tar.gz, .tar.bz, .tar.bz2, .tar.xz, .tar.lz, .tar.lzma, .tar.zip
      .zip.*²: .zip.gz, .zip.bz, .zip.bz2, .zip.xz, .zip.lz, .zip.lzma, .zip.zip
```


## Installation

### Getting a pre-compiled binary

```bash
curl -s https://raw.githubusercontent.com/vrmiguel/ouch/master/install.sh | bash
```

### Building

A recent [Rust] toolchain is needed to build `ouch`. Go to [rustup.rs](https://rustup.rs/) to get it.

Once [Cargo](https://doc.rust-lang.org/cargo/) is installed, run:

```bash
cargo install ouch
# or 
git clone https://github.com/vrmiguel/ouch
cargo install --path ouch
```

## Supported operating systems



## Limitations

`ouch` does encoding and decoding in-memory, so decompressing very large files with it is not advisable.
