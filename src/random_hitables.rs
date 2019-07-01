use rand;
use rand::prelude::*;

use crate::color3::Color3;
use crate::vec3::Vec3;
use crate::hitable::Hitable;
use crate::list_hitable::ListHitable;
use crate::sphere_hitable::SphereHitable;
use crate::material::{LambertMaterial, MetalMaterial};

use crate::material::DielectricMaterial;

// Book cover on the book of Ray Tracing in One Weekend
pub fn iow_book_cover(rng: &mut rand::rngs::StdRng) -> impl Hitable {
    let mut hitables: Vec<SphereHitable> = Vec::new();

    hitables.push(SphereHitable {
        center: Vec3{x: 0.0, y: -1000.0, z: 0.0},
        radius: 1000.0,
        material: Box::new(LambertMaterial{albedo: Color3{r: 0.5, g: 0.5, b: 0.5}})
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center: Vec3 = Vec3 {
                x: a as f32 + 0.9 * rng.gen::<f32>(),
                y: 0.2,
                z: b as f32 + 0.9 * rng.gen::<f32>()
            };

            if (&center - &Vec3{x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                if choose_mat < 0.8 {
                    hitables.push(SphereHitable {
                        center,
                        radius: 0.2,
                        material: Box::new(LambertMaterial{
                            albedo: Color3{
                                r: rng.gen::<f32>() * rng.gen::<f32>(),
                                g: rng.gen::<f32>() * rng.gen::<f32>(),
                                b: rng.gen::<f32>() * rng.gen::<f32>()
                            }
                        })
                    });
                } else if choose_mat < 0.95 {
                    hitables.push(SphereHitable {
                        center,
                        radius: 0.2,
                        material: Box::new(MetalMaterial{
                            albedo: Color3{
                                r: 0.5 * (1.0 + rng.gen::<f32>()),
                                g: 0.5 * (1.0 + rng.gen::<f32>()),
                                b: 0.5 * (1.0 + rng.gen::<f32>())
                            },
                            f: 0.5 * rng.gen::<f32>()
                        })
                    });
                } else {
                    hitables.push(SphereHitable {
                        center,
                        radius: 0.2,
                        material: Box::new(DielectricMaterial{ref_idx: 1.5})
                    });
                }
            }
        }
    }

    hitables.push(SphereHitable {
        center: Vec3{x: 0.0, y: 1.0, z: 0.0},
        radius: 1.0,
        material: Box::new(DielectricMaterial{ref_idx: 1.5})
    });
    hitables.push(SphereHitable {
        center: Vec3{x: -4.0, y: 1.0, z: 0.0},
        radius: 1.0,
        material: Box::new(LambertMaterial{albedo: Color3{r: 0.4, g: 0.2, b: 0.1}})
    });
    hitables.push(SphereHitable {
        center: Vec3{x: 4.0, y: 1.0, z: 0.0},
        radius: 1.0,
        material: Box::new(MetalMaterial{albedo: Color3{r: 0.7, g: 0.6, b: 0.5}, f: 0.0})
    });

    ListHitable{hitables}
}
