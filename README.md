# Perform

Perform is a placeholder name for a benchmarking platform that's currently very
much a WIP.

## Features

Perform has a number of features that set it apart from other benchmarking
tools:

* **Historical data** — Perform stores results *across* many benchmark runs and
  enables you to query historical results. This provides you with a single place
  to discern patterns.
* **Unified web interface** — Perform provides a web app that enables you to
  browse benchmarking results, visualizations both within a bench and across
  multiple runs, and easy-to-understand figures like KPIs. 
* **API** — Benchmarking results can be queries via a [GraphQL] API.

## Data model

In Perform, benches are identified by a slug (like **v1_2** or
**api_disabled**). This provides the basic "key" for organizing results. You can
also attach any arbitrary key/value metadata you like to benches, such as info
about programming languages, runtimes, operating systems, etc.

A bench has many **runs**, and each run has a series of calculated results
associated with it, including mean, variance, R-squared, and some others. When
you submit the results of a run, Perform calculates relevant results, builds
charts and graphs (as SVGs) and writes them to disk, and then stores those
results.

Perform does *not* store every data point that it sees, at least in its initial
incarnation. Raw data is fodder for calculated results and visualizations that
is discarded as soon as it's no longer needed.

[graphql]: https://graphql.org