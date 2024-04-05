# Dab S3 Logs

## Description

A tool to fetch and output logs from S3 buckets.

## Usage

```
Tool to fetch and output logs from S3 buckets

Usage: dab-s3-logs <COMMAND>

Commands:
  preview  Preview fetch results
  fetch    Fetch logs from S3
  output   Output downloaded logs to stdout
  config   Manage configuration options
  reset    Clear storage directory
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

## Installation

```bash
brew tap NickMoignard/tap;
brew install dab-s3-logs;
```

## Issues

Make sure you've logged in with AWS CLI and have the necessary permissions to access the S3 bucket. (also check `AWS_PROFILE` make sure it is the correct profile)
