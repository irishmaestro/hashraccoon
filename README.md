# hashraccoon

![raccoon](https://github.com/IrishMaestro/hashraccoon/raw/master/raccoon.jpeg "raccoon")

## Installation
### Clone the repository and run the install.sh bash script
`git clone https://github.com/IrishMaestro/hashraccoon.git`

`cd hashraccoon`

`chmod +x install.sh`

`./install.sh`

### Compile the code
`cargo build --release`

### Run hashraccoon
`./target/release/hashraccoon <algorithm> <hash>`

E.g.

`./target/release/hashraccoon md5 fc5e038d38a57032085441e7fe7010b0`

`./target/release/hashraccoon sha256 936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af`

