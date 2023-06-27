# fusion-calculator-rs

A simple CLI tool to calculate fusions for Persona 5 Royal written in Rust. Essentially an efficient, CLI version of [chinhodado's `persona5_calculator`](https://github.com/chinhodado/persona5_calculator).

## Example usage

### Find the fusion between two Personas

```bash
tyler@debbie:~$ fusion-calculator fuse calculate --first "Jack Frost" --second "Thor"
╭────────┬───────┬───────────╮
│ Result │ Level │   Arcana  │
╞════════╪═══════╪═══════════╡
│ Isis   │ 26    │ Priestess │
╰────────┴───────┴───────────╯
tyler@debbie:~$ 
```

### Find possible fusions from a given Persona

```bash
tyler@debbie:~$ fusion-calculator fuse from --name "Jack Frost"
╭────────────────────────┬───────┬────────────┬──────────────────┬───────┬────────────╮
│       Fused With       │ Level │   Arcana   │      Result      │ Level │   Arcana   │
╞════════════════════════╪═══════╪════════════╪══════════════════╪═══════╪════════════╡
│ Jack-o'-Lantern        │ 2     │ Magician   │ Cait Sith        │ 5     │ Magician   │
├────────────────────────┼───────┼────────────┼──────────────────┼───────┼────────────┤
│ Pixie                  │ 2     │ Lovers     │ Incubus          │ 5     │ Devil      │
├────────────────────────┼───────┼────────────┼──────────────────┼───────┼────────────┤
│ Agathion               │ 3     │ Chariot    │ Silky            │ 6     │ Priestess  │
├────────────────────────┼───────┼────────────┼──────────────────┼───────┼────────────┤
│ Mandrake               │ 3     │ Death      │ Bicorn           │ 4     │ Hermit     │
├────────────────────────┼───────┼────────────┼──────────────────┼───────┼────────────┤
| ...                    | ...   | ...        | ...              | ...   | ...        |
│ Maria                  │ 93    │ Faith      │ Valkyrie         │ 44    │ Strength   │
╰────────────────────────┴───────┴────────────┴──────────────────┴───────┴────────────╯
tyler@debbie:~$ 
```

### Find all possible combinations to fuse a given Persona

```bash
tyler@debbie:~$ fusion-calculator fuse to --name "Jack Frost"
╭─────────────────┬───────┬──────────┬─────────────────┬───────┬──────────╮
│      First      │ Level │  Arcana  │      Second     │ Level │  Aracana │
╞═════════════════╪═══════╪══════════╪═════════════════╪═══════╪══════════╡
│ Jack-o'-Lantern │ 2     │ Magician │ Sandman         │ 23    │ Magician │
├─────────────────┼───────┼──────────┼─────────────────┼───────┼──────────┤
│ Jack-o'-Lantern │ 2     │ Magician │ Choronzon       │ 28    │ Magician │
├─────────────────┼───────┼──────────┼─────────────────┼───────┼──────────┤
│ Cait Sith       │ 5     │ Magician │ Nekomata        │ 17    │ Magician │
├─────────────────┼───────┼──────────┼─────────────────┼───────┼──────────┤
│ Cait Sith       │ 5     │ Magician │ Sandman         │ 23    │ Magician │
├─────────────────┼───────┼──────────┼─────────────────┼───────┼──────────┤
│ Jack Frost      │ 11    │ Magician │ Jack Frost      │ 11    │ Magician │
├─────────────────┼───────┼──────────┼─────────────────┼───────┼──────────┤
│ Jack Frost      │ 11    │ Magician │ Nekomata        │ 17    │ Magician │
├─────────────────┼───────┼──────────┼─────────────────┼───────┼──────────┤
│ Nekomata        │ 17    │ Magician │ Cait Sith       │ 5     │ Magician │
├─────────────────┼───────┼──────────┼─────────────────┼───────┼──────────┤
│ Nekomata        │ 17    │ Magician │ Jack Frost      │ 11    │ Magician │
├─────────────────┼───────┼──────────┼─────────────────┼───────┼──────────┤
│ Sandman         │ 23    │ Magician │ Jack-o'-Lantern │ 2     │ Magician │
├─────────────────┼───────┼──────────┼─────────────────┼───────┼──────────┤
│ Sandman         │ 23    │ Magician │ Cait Sith       │ 5     │ Magician │
├─────────────────┼───────┼──────────┼─────────────────┼───────┼──────────┤
│ Choronzon       │ 28    │ Magician │ Jack-o'-Lantern │ 2     │ Magician │
╰─────────────────┴───────┴──────────┴─────────────────┴───────┴──────────╯
tyler@debbie:~$ 
```

### List personas, with filters

#### By specific Arcanas

```bash
tyler@debbie:~$ fusion-calculator list personas --arcanas Magician --arcanas Death
╭─────────────────┬───────┬──────────╮
│       Name      │ Level │  Arcana  │
╞═════════════════╪═══════╪══════════╡
│ Mandrake        │ 3     │ Death    │
├─────────────────┼───────┼──────────┤
│ Mokoi           │ 9     │ Death    │
├─────────────────┼───────┼──────────┤
│ Matador         │ 17    │ Death    │
├─────────────────┼───────┼──────────┤
│ Nue             │ 20    │ Death    │
├─────────────────┼───────┼──────────┤
│ Pisaca          │ 28    │ Death    │
├─────────────────┼───────┼──────────┤
│ Hell Biker      │ 37    │ Death    │
├─────────────────┼───────┼──────────┤
│ Hope Diamond    │ 40    │ Death    │
├─────────────────┼───────┼──────────┤
│ Pale Rider      │ 54    │ Death    │
├─────────────────┼───────┼──────────┤
│ Chernobog       │ 62    │ Death    │
├─────────────────┼───────┼──────────┤
│ Thanatos        │ 65    │ Death    │
├─────────────────┼───────┼──────────┤
│ Thanatos Picaro │ 69    │ Death    │
├─────────────────┼───────┼──────────┤
│ Mot             │ 72    │ Death    │
├─────────────────┼───────┼──────────┤
│ Alice           │ 83    │ Death    │
├─────────────────┼───────┼──────────┤
│ Jack-o'-Lantern │ 2     │ Magician │
├─────────────────┼───────┼──────────┤
│ Cait Sith       │ 5     │ Magician │
├─────────────────┼───────┼──────────┤
│ Jack Frost      │ 11    │ Magician │
├─────────────────┼───────┼──────────┤
│ Nekomata        │ 17    │ Magician │
├─────────────────┼───────┼──────────┤
│ Sandman         │ 23    │ Magician │
├─────────────────┼───────┼──────────┤
│ Choronzon       │ 28    │ Magician │
├─────────────────┼───────┼──────────┤
│ Queen Mab       │ 43    │ Magician │
├─────────────────┼───────┼──────────┤
│ Rangda          │ 48    │ Magician │
├─────────────────┼───────┼──────────┤
│ Forneus         │ 63    │ Magician │
├─────────────────┼───────┼──────────┤
│ Surt            │ 83    │ Magician │
├─────────────────┼───────┼──────────┤
│ Futsunushi      │ 86    │ Magician │
╰─────────────────┴───────┴──────────╯
tyler@debbie:~$ 
```

#### By minimum level

```bash
tyler@debbie:~$ fusion-calculator list personas --min-level 90
╭────────────────┬───────┬───────────╮
│      Name      │ Level │   Arcana  │
╞════════════════╪═══════╪═══════════╡
│ Maria          │ 93    │ Faith     │
├────────────────┼───────┼───────────┤
│ Satanael       │ 95    │ Fool      │
├────────────────┼───────┼───────────┤
│ Messiah Picaro │ 90    │ Judgement │
├────────────────┼───────┼───────────┤
│ Satan          │ 92    │ Judgement │
├────────────────┼───────┼───────────┤
│ Lucifer        │ 93    │ Star      │
├────────────────┼───────┼───────────┤
│ Mada           │ 90    │ Tower     │
╰────────────────┴───────┴───────────╯
tyler@debbie:~$ 
```

#### By maximum level

```bash
tyler@debbie:~$ fusion-calculator list personas --max-level 5
╭─────────────────┬───────┬──────────╮
│       Name      │ Level │  Arcana  │
╞═════════════════╪═══════╪══════════╡
│ Agathion        │ 3     │ Chariot  │
├─────────────────┼───────┼──────────┤
│ Mandrake        │ 3     │ Death    │
├─────────────────┼───────┼──────────┤
│ Incubus         │ 5     │ Devil    │
├─────────────────┼───────┼──────────┤
│ Arsene          │ 1     │ Fool     │
├─────────────────┼───────┼──────────┤
│ Bicorn          │ 4     │ Hermit   │
├─────────────────┼───────┼──────────┤
│ Pixie           │ 2     │ Lovers   │
├─────────────────┼───────┼──────────┤
│ Jack-o'-Lantern │ 2     │ Magician │
├─────────────────┼───────┼──────────┤
│ Cait Sith       │ 5     │ Magician │
╰─────────────────┴───────┴──────────╯
tyler@debbie:~$ 
```

#### By a combination of filters 

```bash
tyler@debbie:~$ fusion-calculator list personas --arcanas Death --arcanas Magician --min-level 20 --max-level 25
╭─────────┬───────┬──────────╮
│   Name  │ Level │  Arcana  │
╞═════════╪═══════╪══════════╡
│ Nue     │ 20    │ Death    │
├─────────┼───────┼──────────┤
│ Sandman │ 23    │ Magician │
╰─────────┴───────┴──────────╯
tyler@debbie:~$ 
```

## Other features

### Output as JSON

`fusion-calculator-rs` allows you to override the `--output-type` to JSON, either for writing out data or even piping to a program like `jq`. Example:

```bash
# Show all Personas that can be fused with Jack Frost and have a level greater than 90
tyler@debbie:~$ fusion-calculator fuse from --name "Jack Frost" --output-type json | jq '.[] | select(.first.data.level > 90)'
{
  "first": {
    "name": "Satan",
    "data": {
      "level": 92,
      "arcana": "Judgement"
    }
  },
  "second": {
    "name": "Valkyrie",
    "data": {
      "level": 44,
      "arcana": "Strength"
    }
  }
}
{
  "first": {
    "name": "Lucifer",
    "data": {
      "level": 93,
      "arcana": "Star"
    }
  },
  "second": {
    "name": "Skadi",
    "data": {
      "level": 53,
      "arcana": "Priestess"
    }
  }
}
{
  "first": {
    "name": "Maria",
    "data": {
      "level": 93,
      "arcana": "Faith"
    }
  },
  "second": {
    "name": "Valkyrie",
    "data": {
      "level": 44,
      "arcana": "Strength"
    }
  }
}
tyler@debbie:~$ 
```

## Installation

Simply clone the repo:

```bash
git clone https://github.com/tylerburdsall/fusion-calculator-rs && cd fusion-calculator-rs
```

Build:

```
cargo build --release
```

And place anywhere you'd like in your `$PATH`

## Roadmap

This isn't necessarily a commitment, but some things like I'd like to add one day:

* Support for vanilla Persona 5
* Support for previous Persona games and their iterations
* Add actual data to each Persona (e.g. stats, movesets)
* Better filters
* Support running this behind an API (see below for more)
* And more!

## Running as a backend server

If, for some reason, someone wanted to support this as an API, there are definitely a lot of optimizations that could be made to further boost performance. If it were me, I would do the following:

* Memoize results
* Store static data behind an `Arc` so threads can access the data without copying it per-thread
* Reduce the amount of `.to_string()` calls are made and use `Cow` instead

Since this is just a "run once" CLI, none of the above are really necessary from a performance standpoint.

## But why make this when multiple tools (with a UI) exist?

Simply because I wanted to try my hand at this in Rust! I felt the Web UIs were incredibly slow for what they were, even as static web pages.

## License

See [LICENSE](./LICENSE)
