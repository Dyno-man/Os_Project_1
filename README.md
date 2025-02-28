# Library Management System

## Overview

This Library Management System is a Rust-based application that simulates a library's book management operations with a focus on concurrent processing. The system demonstrates various threading and synchronization techniques through different phases of implementation:

- **Phase 1**: Basic threading for borrowing and returning books
- **Phase 2**: Mutex implementation for thread synchronization
- **Phase 3**: Simulating resource contention scenarios
- **Phase 4**: Resolving thread locking issues

The system manages books and users, tracks book borrowing and returning operations, and logs all activities. It also includes functionality to import book data from CSV files.

## Features

- Book management (add, borrow, return)
- User management
- CSV import for book data
- Concurrent operations with thread safety
- Multiple implementation phases demonstrating different concurrency approaches

## Dependencies

- Rust (2021 edition)
- rand = "0.9.0"

## Installation

1. Ensure you have Rust and Cargo installed on your system. If not, install them from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Clone the repository:
   ```
   git clone https://github.com/Dyno-man/Os_Project_1.git
   cd library-management-system
   ```

3. Build the project:
   ```
   cargo build
   ```

## Running the Program

1. Run the application:
   ```
   cargo run
   ```

2. The program will:
   - Load book data from CSV files in the project directory
   - Create sample users
   - Run through the different phases of thread implementation
     
## Project Structure

- `src/main.rs`: Main application entry point and phase implementations
- `src/book.rs`: Core data structures for Book and User
- `src/book_shipment_log.rs`: Functionality for importing books from CSV files

## CSV File Format

The system expects CSV files with the following format:
```
Title,Author,ISBN
"Book Title 1","Author Name 1","ISBN-123456789"
"Book Title 2","Author Name 2","ISBN-987654321"
```

The first line is treated as a header and skipped during import.

## Threading Phases

1. **Phase 1**: Creates 100 threads each for borrowing and returning books without synchronization
2. **Phase 2**: Implements mutex locks to prevent concurrent access to the library
3. **Phase 3**: Simulates resource contention scenarios
4. **Phase 4**: Resolves thread locking issues with improved synchronization


## License

Apacha 2.0
