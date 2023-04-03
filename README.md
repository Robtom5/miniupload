# miniupload
Simple rust cli tool for uploading to a miniserve instance

## Usage

`miniupload upload <FILE>` will upload the file to configured url and folder

`miniupload download <FILENAME> <DEST>` will download the file from the remote if it exists to the provided destination.

`miniupload -h` for more information.

## Configuration

The target url and subfolder on the server can be configured by running `miniupload config -a <URL> -f <FOLDER>`. Alternatively, the target and folder can be set as environment variables, `MINIUPLOAD_TARGET` and `MINIUPLOAD_FOLDER` respectively.

NB: The values stored in the environment variables will always be used if set,

## See Also
[miniserve](https://github.com/svenstaro/miniserve)