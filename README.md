# pasta

A cli based pastebin in Rust, but very insecure

### Use nightly toolchain to build

```sh
rustup override set nightly
```

When this program is running, you can use `curl` to post data.

Assuming the address is `0.0.0.0` and port is `8000`, you can do the following:

#### Posting data

```sh
curl --data-binary value="$(cat /path/to/file)"  http://0.0.0.0:8000
```

This prints a code that can be used to retrieve your file.

#### Retrieving files

Visit `http://address:port/code` from a web browser, or send a get request to the link. Assuming the code obtained from submitting your data was `GaFH`, visit `http://0.0.0.0:8000/GaFH` from a web browser or send a simple get request.

```sh
curl http://0.0.0.0:8000/GaFH
```

Files are stored to disk, so they presist even after application restart.
