# Financial Transaction Parser

A Rust library for parsing and processing financial transaction data from multiple file formats including CSV, TXT, and
binary files.

## Description

This library provides functionality to parse, serialize, and process financial transaction records. It supports
different transaction types (deposits, withdrawals, transfers) and handles various transaction attributes including IDs,
amounts, timestamps, statuses, and descriptions.

## Features

- Multiple format support (CSV, TXT, Binary)
- Flexible parser factory system
- Transaction type validation
- Error handling for parsing operations
- Support for reading and writing transactions

## Transaction Types

The following transaction types are supported:

- DEPOSIT: Adding funds
- WITHDRAWAL: Removing funds
- TRANSFER: Moving funds between accounts

## Transaction Fields

Each transaction includes:

- Transaction ID
- Transaction Type
- From User ID
- To User ID
- Amount
- Timestamp
- Status (Success/Failure/Pending)
- Description

## Usage

```bash
converter --input <INPUT> --output <OUTPUT> --input-format <INPUT_FORMAT> --output-format <OUTPUT_FORMAT>
```

```bash
comparer --file1 <FILE1> --file2 <FILE2> --format1 <FORMAT1> --format2 <FORMAT2>
```