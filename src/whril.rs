use std::{fs::File, io::{BufWriter, Write}};

use crate::glsl::{Vec2, Vec3, Vec4};

pub fn run() -> std::io::Result<()> {
    for i in 0..1 {
        let output_path = format!("output/output-{:02}.ppm", i);
        let file = File::create(&output_path)?;
        let mut buf = BufWriter::new(file);

        let width: i32 = 16 * 160;
        let height: i32 = 9 * 160;

        let r = Vec2::new(width as f32, height as f32);
        let t = (i as f32 / 240.0) * std::f32::consts::TAU;

        writeln!(buf, "P6")?;
        writeln!(buf, "{} {}", width, height)?;
        writeln!(buf, "255")?;

        let mut pixel;
        for x in 0..height {
            for y in 0..width {

                // Shader code made by XorDev
                //
                // https://x.com/XorDev/status/1986071686785986848?s=20
                //
                // for(float i,z,d,h;i++<8e1;o+=vec4(9,5,h+t,1)/d)
                // {
                // vec3 p = z*normalize(FC.rgb*2.-r.xyy),a;
                // p.z+=9.;
                // a=mix(dot(a+=.5,p)*a,p,sin(h=dot(p,p/p)-t))+cos(h)*cross(a,p);
                // for(d=0.;d++<9.;a+=.3*sin(a*d).zxy);z+=d=length(a.xz)/15.;
                // }
                // o=tanh(o/1e4);

                let fc = Vec4::new(x as f32, y as f32, 0.0, 1.0);
                let mut o = Vec4::new(0.0, 0.0, 0.0, 0.0);
                let mut e = 0.0;
                let mut z = 1.0;
                let mut h;

                while {
                    e += 1.0;
                    e < 80.0
                } {
                    let mut p = (fc.rgb() * 2.0 - r.xyy()).normalize();
                    p = p * z;
                    let mut a = Vec3::new(0.0, 0.0, 0.0);
                    p.z += 9.0;

                    a = {
                        a = a + 0.5;
                        h = Vec3::dot(p, p / p) - t;

                        let mix_t = h.sin();
                        let mixed = Vec3::mix(
                            a * Vec3::dot(a, p),
                            p,
                            mix_t
                        );

                        (Vec3::cross(a, p) * mix_t.cos()) + mixed
                    };

                    let mut d = 0.0;
                    while {
                        d += 1.0;
                        d < 9.0
                    } {
                        let v = (a * d).map(f32::sin).zxy();
                        a = a + v * 0.3;
                    }

                    d = a.xz().length() / 15.0;
                    z += d;


                    o = o + Vec4::new(9.0, 5.0, h + t, 1.0) / d;
                }
                
                o = (o / 10000.0).tanh();

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
