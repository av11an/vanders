use std::{fs::File, io::{BufWriter, Write}};

use crate::glsl::{Vec2, Vec4};

pub fn run() -> std::io::Result<()> {
    for i in 0..60*5 {
        let output_path = format!("output/output-{:02}.ppm", i);
        let file = File::create(&output_path)?;
        let mut buf = BufWriter::new(file);

        let width: i32 = 16 * 60;
        let height: i32 = 9 * 60;

        let r = Vec2::new(width as f32, height as f32);
        let t = (i as f32 / 240.0) * std::f32::consts::TAU;

        writeln!(buf, "P6")?;
        writeln!(buf, "{} {}", width, height)?;
        writeln!(buf, "255")?;

        let mut pixel;
        for x in 0..height {
            for y in 0..width {

                // Interesting Shader code made by @XorDev
                // https://x.com/XorDev/status/1894494786368745715?s=20

                // vec2 p = (FC.multiply(2.-r))/r.y
                // vec2 l, i, v = p*(l+=4.-4.*abs(.7-dot(p,p)));

                // for(;i.y++<8.;o+=(sin(v.xyyx)+1.) * abs(v.x-v.y))
                //     v+=cos(v.yx*i.y+i+t)/i.y+.7;

                // o=tanh(5.*exp(l.x-4.-p.y*vec4(-1,1,2,0))/o);

                // End Shader code

                let fc = Vec2::new(x as f32, y as f32);
                let p = (fc * 2.0 - r) / r.y;
                let mut o = Vec4::new(0.0, 0.0, 0.0, 0.0);
                let mut l = Vec2::new(0.0, 0.0);
                let mut e = Vec2::new(0.0, 0.0);

                l = l + (4.0 - 4.0 * (0.7 - Vec2::dot(p, p)).abs());
                let mut v = p * l;

                while {
                    e.y += 1.0;
                    e.y < 8.0
                } {
                    let sin_term = v.xyyx().map(|x| x.sin());
                    let one = Vec4::new(1.0, 1.0, 1.0, 1.0);
                    let diff = (v.x - v.y).abs();

                    o = o + ((sin_term + one) * diff);
                    
                    let arg = v.yx() * e.y + Vec2::new(e.x, e.y) + t;
                    let c = Vec2::new(arg.x.cos(), arg.y.cos());
                     
                    v = v + c / e.y + 0.7;
                }

                let exp_term = (Vec4::new(-1.0, 1.0, 2.0, 0.0) * (l.x - 4.0 - p.y)).exp();

                o = (exp_term * 5.0).tanh() / o;

                pixel = [
                    clamp(o.x),
                    clamp(o.y),
                    clamp(o.z)
                ];
                buf.write_all(&pixel)?;

            }
        }

        println!("Generated {}\n", &output_path);
    }

    Ok(())
}

fn clamp(x: f32) -> u8 {
    (x.clamp(0.0, 1.0) * 255.0) as u8
}
