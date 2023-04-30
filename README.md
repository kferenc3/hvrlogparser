# HVR Log Parser

A Python-based command-line utility to parse logfiles generated by HVR/Fivetran LDP.

## Usage

```
hvrlogparser [OPTIONS] --method <METHOD> <INPUT_FILE>
```

## Arguments

* `<INPUT_FILE>`: The input logfile to be parsed.

## Options

* `-m, --method <METHOD>`: The parsing method. Possible values are `lines`, `date`, and `bytes`. Default value is `lines`.
* `-g, --granularity <GRANULARITY>`: Required if the parsing method is `date`. Possible values are `minute`, `hour`, `day`, and `month`. Default value is `hour`.
* `-b, --begin-time <BEGIN_TIME>`: Specify the starting point for parsing. The format of the timestamp should be `YYYY-mm-ddTHH:mm:SS+00:00`. Default value is the beginning of the file.
* `-e, --end-time <END_TIME>`: Specify the ending point for parsing. The format of the timestamp should be `YYYY-mm-ddTHH:mm:SS+00:00`. Default value is the end of the file.
* `-l, --lower-bound <LOWER_BOUND>`: Optional if the parsing method is `lines` or `bytes`. The starting line or byte to begin parsing from.
* `-u, --upper-bound <UPPER_BOUND>`: Optional if the parsing method is `lines` or `bytes`. The final line or byte to end parsing at.
* `-c, --chunk-size <CHUNK_SIZE>`: Required if the parsing method is `lines` or `bytes`. The number of lines in a chunk or the number of bytes in a chunk, depending on the selected method. Default value is `10`.
* `-f, --file-basename <FILE_BASENAME>`: The basename of the output files. Default value is `part_`.
* `-h, --help`: Print the help message.
* `-V, --version`: Print the version of the utility.
