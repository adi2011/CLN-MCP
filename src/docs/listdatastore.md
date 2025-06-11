lightning-listdatastore -- Command for listing (plugin) data
**listdatastore** [*key*] 
The **listdatastore** RPC command allows plugins to fetch data which was stored in the Core Lightning database.

- **key** (one of, optional):
  - (array of strings): All immediate children of the *key* (or root children) are returned.
    Using the first element of the key as the plugin name (e.g. `[ 'summary' ]`) is recommended.
    An array of values to form a hierarchy (though a single value is treated as a one-element array).
    - (string, optional)
  - (string)
