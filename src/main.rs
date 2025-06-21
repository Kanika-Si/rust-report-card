use std::fs::File;
use std::io;
use pdf_canvas::{Pdf, BuiltinFont};

fn calculate_average(total_marks: f32, num_subjects: u32) -> f32 {
    total_marks / num_subjects as f32
}

fn assign_grade(average: f32) -> &'static str {
    match average {
        x if x >= 90.0 => "A",
        x if x >= 75.0 => "B",
        x if x >= 60.0 => "C",
        _ => "D",
    }
}

fn generate_pdf(name: &str, total: f32, average: f32, grade: &str) {
    let path = "output/report_card.pdf";
    std::fs::create_dir_all("output").expect("Could not create output folder");

    let file = File::create(path).expect("Could not create file");
    let mut doc = Pdf::new(file).expect("Failed to initialize PDF");

    let font = BuiltinFont::Helvetica;

    doc.render_page(595.0, 842.0, |canvas| {
        // Title
        canvas.left_text(50.0, 800.0, font, 20.0, "📄 Student Report Card")?;

        // Content
        canvas.left_text(50.0, 760.0, font, 14.0, &format!("Name: {}", name))?;
        canvas.left_text(50.0, 740.0, font, 14.0, &format!("Total Marks: {:.2}", total))?;
        canvas.left_text(50.0, 720.0, font, 14.0, &format!("Average Marks: {:.2}", average))?;
        canvas.left_text(50.0, 700.0, font, 14.0, &format!("Grade: {}", grade))?;

        Ok(())
    }).expect("Failed to render page");

    doc.finish().expect("Failed to save PDF");
    println!("✅ PDF generated at: {}", path);
}

fn main() {
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut num_subjects = String::new();

    println!("🎓 Enter student name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("🧮 Enter total marks:");
    io::stdin().read_line(&mut total_marks).unwrap();

    println!("📚 Enter number of subjects:");
    io::stdin().read_line(&mut num_subjects).unwrap();

    let name = name.trim();
    let total: f32 = total_marks.trim().parse().unwrap();
    let num_subjects: u32 = num_subjects.trim().parse().unwrap();

    let average = calculate_average(total, num_subjects);
    let grade = assign_grade(average);

    generate_pdf(name, total, average, grade);
}
