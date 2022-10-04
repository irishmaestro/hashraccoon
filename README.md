# hashraccoon
![GitHub Repo stars](https://img.shields.io/github/stars/irishmaestro/hashraccoon?color=black&style=for-the-badge)
![Crates.io](https://img.shields.io/crates/d/hashraccoon?color=black&style=for-the-badge)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/irishmaestro/hashraccoon?color=black&style=for-the-badge)
![GitHub](https://img.shields.io/github/license/irishmaestro/hashraccoon?color=black&style=for-the-badge)

![hrgh](https://user-images.githubusercontent.com/70972101/188287035-6ac7221e-4244-4f1b-af79-b03bbf24aaa7.gif)

## Installation
### Install cargo
```shell 
curl https://sh.rustup.rs -sSf | sh
```
### Install the hashraccoon [crate](https://crates.io/crates/hashraccoon "crate")
```shell
cargo install hashraccoon
```

### Download the [rockyou](https://github.com/IrishMaestro/rockyou "rockyou") wordlist 
```shell
wget https://github.com/IrishMaestro/rockyou/raw/master/rockyou.txt.gz
gunzip rockyou.txt.gz
```

### Run hashraccoon
```shell
hashraccoon <algorithm> <hash> <path/to/wordlist>
```

### Examples
```shell
hashraccoon md5 fd3dd4b168a1cae43f5b329142f73d27 $(pwd)/rockyou.txt
```

```shell
hashraccoon sha256 dc11a50ee7b0763306d2019e611d1ed3e66b3f182b3b2c18acbacccfd0ba656b $(pwd)/rockyou.txt
```

https://user-images.githubusercontent.com/70972101/188251119-e2a6bb54-9edf-412f-98c1-60c54eef56ab.mov
