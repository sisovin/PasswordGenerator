# Randomized Password Generator

This project generates a random password using Rust. Below is the corrected and functional implementation.

---

### **Corrected Code**

```rust
use rand::Rng;

fn main() {
    let password_length = 15;
    let mut result = String::new();

    for _ in 0..password_length {
        let number = rand::thread_rng().gen_range(33..127);
        result.push(char::from(number as u8));
    }

    println!("Generated Password: {}", result);
}
```

---

### **Code Structure Improvements**

1. **Imports**:
   - Import the `rand::Rng` trait for random number generation.

2. **Password Length**:
   - Set the password length to `15`.

3. **Result String Initialization**:
   - Initialize an empty string to store the password.

4. **Random Character Generation**:
   - Use the range `33..127` for printable ASCII characters.
   - Convert the generated number to a character.

5. **Loop Syntax**:
   - Iterate `password_length` times to generate each character.

6. **Appending Characters**:
   - Append characters to the result string.

7. **Output**:
   - Print the generated password.

---

### **Sample Output**

```
Generated Password: x<1PY@9>7B~%h0$
```

---

### **Next Steps**

1. **Customizable Character Sets**:
   - Include options for lowercase, uppercase, digits, and special characters.

2. **Prevent Repetition or Ensure Diversity**:
   - Avoid repeating characters too frequently.

3. **User-Defined Length**:
   - Allow users to specify the password length.

4. **Use Stronger Random Sources**:
   - Consider using `rand::rngs::OsRng` for cryptographically secure passwords.

5. **Save Passwords to a File**:
   - Add functionality to save generated passwords to a file.

6. **Interactive Mode**:
   - Create an interactive mode for user preferences.

7. **Unit Tests**:
   - Write unit tests to ensure the correctness of the password generation logic.

### **How to Run**

To format, lint, run, and generate documentation for the project, use the following commands:

```bash
cargo fmt
rustup component add clippy
cargo clippy
cargo run --quiet --release
cargo doc --open
```
