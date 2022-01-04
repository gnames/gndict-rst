# gndict

gndict creates gnfinder dictionaries using gnindex database. The database has
to be created by the latest version of
[gnidump](https://github.com/gnames/gnidump/). It uses an internal gnindex
database and, therefore has an internal use. If you want to modify gnfinder
dictionaries, please add a ticket and I will tell how it can be used without
database access.

## Install and usage

It requres [Rust to be installed](https://www.rust-lang.org/tools/install) on
the system.

```bash
git clone git@github.com:gnames/gndict.git
cd gndict
cp .env.example .env
# modify .env according to database position.
cargo run -- -r
# if data are downloaded already and did not change
cargo run
```

To modify gnfinder dictionaries you can edit dictionaries precursors in `data`
directory.

* ``common-eu-words.txt`` contains normal words and is used to greate "grey"
  dictionaries for species, genera and uninomials.

* ``species_black.txt`` contains words that wrongly appeaar in databases and
  parsed as specific epithets.

* ``uninomials-black.txt`` contains words that wrongly appear in datbases
  and parsed as uninomial or generic words.

After the program runs it will generate new dictionaries in provided by user
``WORK_DIR`` in subdicectory ``dict``. The content of ``dict`` dir will have to
be copied to ``gnfinder/data/files``.

## Index to Organism Names (ION)

Directory `data` has `ion-names.txt` file. It contains scientific names from
ION that do not exist in other data-sources. This list contains a significant
number of misspellings. We will use it anyway because it is important to find
names that are now "out of circulation".
