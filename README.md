# serde-bayard

This is a research project for how to [serde](https://github.com/serde-rs/serde) a document from [bayard](https://github.com/bayard-search/bayard) full text search engine.

## Background

In my [EvaQL](https://github.com/brainvader/EvaQL)  project, data requested from a graphql client (graphiql) is

1. got from bayard serach engine as json string
2. deserialized into GraphQL Object in Juniper
3. sent back to the client

serde_json can treat any json string without special cares. But it's not enough in my project.

Bayard returns json like below;

```json
{
  "_id": [
    "1"
  ],
  "name": [
    "Ikari Shinji"
  ],
  "episode": [
    "/episode/Jo",
    "/episode/Ha",
    "/episode/Q"
  ]
}
```

What I want during deserialization are

* Unwrap array
* Map hierarchical facet in Bayard into Enum type in Rust

It's not impossible to achieve from serde_json default settings.