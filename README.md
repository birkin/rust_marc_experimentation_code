
- next...
    √ add comments to each step of exp02_read_marc_file load_records()
    - pull out bib and/or item-number also.
        --> see marc_cli query.rs query_subfield() to grok and implement getting at sub-fields
    - process a directory instead of file directly.

- eventually...
    - consider appending output to a file instead of storing all data in memory.
    - play w/tokio, certainly to process multiple files...
        - but even possibly to process parts of a file.

---

- goal play:
    - first, to load a file of marc-records, and create a list of titles.
    - then, to do the same for a path to a directory of marc files.

- flow...
    - played around with rust-marc but was having trouble pulling out specific data.
    - so am now experimenting with how marc_cli calls rust-marc.

---

Resources:
- <https://github.com/blackbeam/rust-marc>
- <https://github.com/lannonbr/marc_cli>
- <https://github.com/JacobSandin/marc_21>

---
