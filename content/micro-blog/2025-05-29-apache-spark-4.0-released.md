---
title: ⚡️Apache Spark 4.0 released
blog_section:
  - micro-blog
publish_branch: main
date: "2025-05-29"
description: Apache Spark 4.0 released
link: https://spark.apache.org/releases/spark-release-4-0-0.html
tags:
  - software-engineering
---

[Apache Spark 4.0](https://spark.apache.org/releases/spark-release-4-0-0.html) has been released. It's the first major version update since Spark 3.0 in 2020.

Here's some of the highlights I'm excited about:
- A [new SQL pipe syntax](https://issues.apache.org/jira/browse/SPARK-49555). It seems to be a trend with modern SQL engines to include "pipe" syntax support now (e.g. [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax)). I'm a fan of functional programming inspired design patterns and the excellent work by the [prql](https://prql-lang.org) team, so I'm glad to see this next evolution of SQL play out.
- A [structured logging framework](https://issues.apache.org/jira/browse/SPARK-47240). Spark logs are notoriously lengthy and this means you can now use Spark to consume Spark logs! Coupled with [improvements to](https://issues.apache.org/jira/browse/SPARK-45673) [stacktraces in PySpark](https://issues.apache.org/jira/browse/SPARK-47274), hopefully this will mean less `grep`ping tortuously long stack traces.
- A new [`DESCRIBE TABLE AS JSON`](https://issues.apache.org/jira/browse/SPARK-50541) option. I really dislike unstructured command line outputs that you have to parse with `awk`ward bashisms. JSON input/outputs and manipulation with [`jq`](https://abstractnonsense.xyz/micro-blog/2025-02-18-surely-youre-jqing/) is a far more expressive consumption pattern that I feel captures the spirit of command line processing.
- A [new PySpark Plotting API](https://issues.apache.org/jira/browse/SPARK-49530)! It's interesting to see it supports [plotly](https://github.com/plotly) on the backend as an engine. I'll be curious to see how this plays out going forward... Being able to do #BigData ETL as well as visualisation and analytics within the one tool is a very powerful combination.
- A new lightweight python-only [Spark Connect PyPi package](https://issues.apache.org/jira/browse/SPARK-47540). Now that Spark Connect is getting more traction, it's nice to be able to `pip install` Spark on small clients without having to ship massive jars around.
- A [bug fix for inaccurate Decimal arithmetic](https://issues.apache.org/jira/browse/SPARK-45786). This is interesting only insofar as it reminds me that even well-established, well-tested, correctness-first, open-source software with industry backing can still be subject to really nasty correctness bugs!

Databricks has some excellent coverage on the [main release](https://www.databricks.com/blog/introducing-apache-spark-40) and the [new pipe syntax](https://www.databricks.com/blog/sql-gets-easier-announcing-new-pipe-syntax) specifically.
