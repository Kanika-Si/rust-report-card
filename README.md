# 🎓 Rust Report Card Generator

A simple Rust console application that:
- Accepts a student’s name, total marks, and number of subjects
- Calculates average marks
- Assigns a grade (A/B/C/D)
- Generates a clean, professional PDF report card using the `pdf-canvas` crate

---

## 📸 Output Sample

The PDF is saved at:

output/report-card.pdf
---

## 📦 Technologies Used

- 🦀 Rust (2021 Edition)
- 🖨 `pdf-canvas = "0.7.0"` for PDF generation
- 🧮 Match expressions and type-safe operations

---

## 💡 Key Rust Concepts

### 🔁 Mutability

Rust variables are **immutable by default**. To make them changeable, use `mut`:

```rust
let mut name = String::new();
io::stdin().read_line(&mut name).unwrap();

###🧠 Ownership
Rust’s memory safety is managed by ownership. Each value has a single owner, and data can be borrowed or moved:

rust
Copy
Edit
let name = name.trim(); // 'name' still valid because we're borrowing via trim()
No garbage collector is needed because Rust automatically frees values when they go out of scope.

 ###🔧 Functions
Rust uses fn to define reusable logic. Example:

rust
Copy
Edit
fn calculate_average(total_marks: f32, num_subjects: u32) -> f32 {
    total_marks / num_subjects as f32
}
Functions help with clean modular code and improve testability.


