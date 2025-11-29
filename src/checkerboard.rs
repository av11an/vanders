use std::{fs::File, io::{BufWriter, Write}};

pub fn run() -> std::io::Result<()> {
    for i in 0..60 {
        let output_path = format!("output/output-{:02}.ppm", i);
        let file = File::create(&output_path)?;
        let mut buf = BufWriter::new(file);

        let width: i32 = 16 * 60;
        let height: i32 = 9 * 60;

        writeln!(buf, "P6")?;
        writeln!(buf, "{} {}", width, height)?;
        writeln!(buf, "255")?;

        let mut pixel;
        for x in 0..height {
            for y in 0..width {
                if ((x + i)/60 + (y + i)/60) % 2 == 0 {
                    pixel = [0xFF, 0x00, 0x00];
                } else {
                    pixel = [0xFF, 0xFF, 0xFF];
                }
                buf.write_all(&pixel)?;

            }
        }

        println!("Generated {}\n", &output_path);
    }

    Ok(())
}
