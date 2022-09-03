# hashraccoon
![GitHub Repo stars](https://img.shields.io/github/stars/irishmaestro/hashraccoon?color=black&style=for-the-badge)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/irishmaestro/hashraccoon?color=black&label=commits&style=for-the-badge)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/irishmaestro/hashraccoon?color=black&style=for-the-badge)
![GitHub](https://img.shields.io/github/license/irishmaestro/hashraccoon?color=black&style=for-the-badge)

![raccoon](https://user-images.githubusercontent.com/70972101/188251139-554fa07d-37e8-4eee-a68c-50369d7f6a23.jpeg)

## Installation
### Clone the repository and run the install.sh script
```shell 
git clone https://github.com/IrishMaestro/hashraccoon.git
```

```shell
cd hashraccoon && chmod +x install.sh && ./install.sh
```

### Compile the code
```shell
cargo build --release
```

### Run hashraccoon
```shell
./target/release/hashraccoon <algorithm> <hash> <path/to/wordlist>
```

### Examples
```shell
./target/release/hashraccoon md5 fd3dd4b168a1cae43f5b329142f73d27 $(pwd)/rockyou.txt
```

```shell
./target/release/hashraccoon sha256 dc11a50ee7b0763306d2019e611d1ed3e66b3f182b3b2c18acbacccfd0ba656b $(pwd)/rockyou.txt
```

https://user-images.githubusercontent.com/70972101/188251119-e2a6bb54-9edf-412f-98c1-60c54eef56ab.mov
