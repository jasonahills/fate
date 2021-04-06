# Fate

Fate is a small command line utility for following up on decisions.  It interactively helps you document and review your decisions, storing the results in an SQLite database.

```shell
fate --db_file /path/to/fate.db init            # initialize database
fate --db_file /path/to/fate.db decide          # make a decision
fate --db_file /path/to/fate.db review --check  # check if any decisions are ready for review
fate --db_file /path/to/fate.db review          # review your decisions
```

## Develop

- Build `cargo build`
- Test `cargo test`

