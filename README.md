# hashraccoon
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/irishmaestro/hashraccoon?style=flat&color=white)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/irishmaestro/hashraccoon?style=flat&color=white&label=commits)
![GitHub](https://img.shields.io/github/license/irishmaestro/hashraccoon?style=flat&color=white)
![GitHub Repo stars](https://img.shields.io/github/stars/irishmaestro/hashraccoon?style=social)

![raccoon](https://github.com/IrishMaestro/hashraccoon/raw/master/raccoon.jpeg "raccoon")

## Installation
### Clone the repository and run the install.sh script
`git clone https://github.com/IrishMaestro/hashraccoon.git`

`cd hashraccoon`

`chmod +x install.sh`

`./install.sh`

### Compile the code
`cargo build --release`

### Run hashraccoon
`./target/release/hashraccoon <algorithm> <hash>`

Examples

`./target/release/hashraccoon md5 fc5e038d38a57032085441e7fe7010b0`

`./target/release/hashraccoon sha256 936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af`

