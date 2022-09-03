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
./target/release/hashraccoon <algorithm> <hash>
```

Examples



https://user-images.githubusercontent.com/70972101/188251119-e2a6bb54-9edf-412f-98c1-60c54eef56ab.mov



```shell
./target/release/hashraccoon md5 fc5e038d38a57032085441e7fe7010b0
```

```shell
./target/release/hashraccoon sha256 936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af
```
