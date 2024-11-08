# Person Serialization and Deserialization

This project provides two programs for serializing and deserializing person data using Protocol Buffers. The `serializer` program converts plain text data into binary format, while the `deserializer` program converts the binary data back to text.

## Prerequisites

- **Rust**: Make sure Rust is installed. If not, you can install it from [https://rust-lang.org](https://rust-lang.org).
- **Protobuf Compiler (protoc)**: Required to compile `.proto` files. You can install it as per instructions [here](https://grpc.io/docs/protoc-installation/).

## Project Structure

```plaintext
person-serialization/
├── Cargo.toml
├── src/
│   ├── main.rs           # Serialization program
│   ├── deserialization.rs # Deserialization program
│   ├── person.rs         # Generated protobuf code
│   └── person.proto      # Protobuf schema
└── target/               # Compiled output files
    └── release/
```

## Getting Started

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd person-serialization
   ```

2. Compile the Protobuf file (`person.proto`) to generate `person.rs`. Run:
   ```bash
   protoc --rust_out=src src/person.proto
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### 1. Serializer Program

The **serializer** reads a text file with comma-separated person details and writes serialized data to a binary file.

- **Input Format**: Each line in the input file should be in the format: `last_name,first_name,date_of_birth`
- **Run the Serializer**:

   ```bash
   cargo run --release --bin serializer -- -i <input_file> -o <output_file>
   ```

   Replace `<input_file>` with the path to your text file (e.g., `data.txt`) and `<output_file>` with the desired binary file (e.g., `serialized_data.bin`).

### 2. Deserializer Program

The **deserializer** reads the serialized binary file created by the serializer and writes it to a new text file.

- **Run the Deserializer**:

   ```bash
   cargo run --release --bin deserializer -- -i <input_file> -o <output_file>
   ```

   Replace `<input_file>` with the path to your binary file (e.g., `serialized_data.bin`) and `<output_file>` with the desired output text file (e.g., `output.txt`).

### Example

1. Run the serializer:

   ```bash
   cargo run --release --bin serializer -- -i person.txt -o serialized_data.bin
   ```

2. Run the deserializer:

   ```bash
   cargo run --release --bin deserializer -- -i serialized_data.bin -o output.txt
   ```

## Notes

- The `person.proto` file defines the structure for `Person` data.
- Ensure that the input file for the serializer has valid, comma-separated values for each person.
