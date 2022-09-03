# hashraccoon
![GitHub Repo stars](https://img.shields.io/github/stars/irishmaestro/hashraccoon?color=black&style=for-the-badge)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/irishmaestro/hashraccoon?color=black&label=commits&style=for-the-badge)
![Crates.io](https://img.shields.io/crates/d/hashraccoon?color=black&style=for-the-badge)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/irishmaestro/hashraccoon?color=black&style=for-the-badge)
![GitHub](https://img.shields.io/github/license/irishmaestro/hashraccoon?color=black&style=for-the-badge)

![hashraccoon](https://user-images.githubusercontent.com/70972101/188285582-fd616091-48ef-46b0-9351-91c78137d613.gif)

## Installation
### Install cargo
```shell 
curl https://sh.rustup.rs -sSf | sh
```
### Install the hashraccoon crate
```shell
cargo install hashraccoon
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
