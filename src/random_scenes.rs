use rand;
use rand::prelude::*;

use crate::color3::Color3;
use crate::vec3::Vec3;
use crate::hitable::Hitable;
use crate::list_hitable::ListHitable;
use crate::sphere_hitable::SphereHitable;
use crate::material::{LambertMaterial, MetalMaterial};
use crate::camera::Camera;
use crate::scene::Scene;
use crate::util;

use crate::material::DielectricMaterial;
use core::borrow::Borrow;

// Book cover on the book of Ray Tracing in One Weekend
pub fn iow_book_cover(rng: &mut rand::rngs::StdRng, width: u32, height: u32) -> Scene {
    let mut hitables: Vec<Box<Hitable + Sync>> = Vec::new();

    hitables.push(Box::new(SphereHitable {
        center: Vec3{x: 0.0, y: -1000.0, z: 0.0},
        radius: 1000.0,
        material: Box::new(LambertMaterial{albedo: Color3{r: 0.5, g: 0.5, b: 0.5}})
    }));

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
                    hitables.push(Box::new(SphereHitable {
                        center,
                        radius: 0.2,
                        material: Box::new(LambertMaterial{
                            albedo: Color3{
                                r: rng.gen::<f32>() * rng.gen::<f32>(),
                                g: rng.gen::<f32>() * rng.gen::<f32>(),
                                b: rng.gen::<f32>() * rng.gen::<f32>()
                            }
                        })
                    }));
                } else if choose_mat < 0.95 {
                    hitables.push(Box::new(SphereHitable {
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
                    }));
                } else {
                    hitables.push(Box::new(SphereHitable {
                        center,
                        radius: 0.2,
                        material: Box::new(DielectricMaterial{ref_idx: 1.5})
                    }));
                }
            }
        }
    }

    hitables.push(Box::new(SphereHitable {
        center: Vec3{x: 0.0, y: 1.0, z: 0.0},
        radius: 1.0,
        material: Box::new(DielectricMaterial{ref_idx: 1.5})
    }));
    hitables.push(Box::new(SphereHitable {
        center: Vec3{x: -4.0, y: 1.0, z: 0.0},
        radius: 1.0,
        material: Box::new(LambertMaterial{albedo: Color3{r: 0.4, g: 0.2, b: 0.1}})
    }));
    hitables.push(Box::new(SphereHitable {
        center: Vec3{x: 4.0, y: 1.0, z: 0.0},
        radius: 1.0,
        material: Box::new(MetalMaterial{albedo: Color3{r: 0.7, g: 0.6, b: 0.5}, f: 0.0})
    }));

    let lookfrom: Vec3 = Vec3 {x: 13.0, y: 2.0, z: 3.0};
    let lookat  : Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
    let focus_dist: f32 = 10.0;
    let aperture  : f32  = 0.1;
    let camera: Camera = Camera {
        lookfrom,
        lookat,
        vup: Vec3 {x: 0.0, y: 1.0, z: 0.0},
        vfov: 20.0,
        aspect: width as f32 / height as f32,
        aperture,
        focus_dist
    };

    Scene {
        camera,
        hitable: Box::new(ListHitable{hitables})
    }
}

pub struct FreeFallAnimation {
    width: u32,
    height: u32,
    dt: f32,
    min_t: f32,
    max_t: f32,
    random_seed: u8,

    // passed time
    t: f32,
    look_from_theta: f32,
    moving_hitable_generators: Vec<MovingHitableGenerator>
}

struct MovingHitableGenerator {
    m: f32,
    k: f32,
    v: f32,
    y: f32,
    sphere_hitable: Box<dyn Fn(f32) -> Box<dyn Hitable + Sync>>
}

impl FreeFallAnimation {
    pub fn new(width: u32, height: u32, dt: f32, min_t: f32, max_t: f32, random_seed: u8) -> FreeFallAnimation {
        let mut rng = util::rng_by_seed(random_seed);

        let moving_hitable_generators = {
            let mut v = Vec::new();

            let mut a: f32 = -20.0;
            while a <= 20.0 {
                let mut b: f32 = -20.0;
                while b <= 20.0 {
                    if [Vec3{x: 4.0, y: 1.0, z: 0.0}, Vec3{x: -4.0, y: 1.0, z: 0.0}, Vec3{x: 0.0, y: 1.0, z: 0.0}].iter().all(|v|
                      (&Vec3{x: a, y: 1.0, z: b} - &v).length() > 1.0 + Self::small_sphere_radius
                    ) {
                        // TODO: Remove
                        println!("{}, {}", a, b);
                        // Find proper x and z
                        let (x, z): (f32, f32) = {
                            let mut x: f32;
                            let mut z: f32;
                            while {
                                let r1: f32 = 0.9 * rng.gen::<f32>();
                                let r2: f32 = 0.9 * rng.gen::<f32>();
                                x = a + r1;
                                z = b + r2;

                                let mut sp = Vec::new();
                                let mut y: f32 = Self::small_sphere_radius;
                                while y <= 4.0 {
                                    sp.push(Vec3{x, y, z});
                                    y += 0.1;
                                }

                                let v = sp.iter().all(|&c|
                                    [Vec3{x: 4.0, y: 1.0, z: 0.0}, Vec3{x: -4.0, y: 1.0, z: 0.0}, Vec3{x: 0.0, y: 1.0, z: 0.0}].iter().all(|v|
                                        (&c - &v).length() > 1.0 + Self::small_sphere_radius
                                    )
                                );
                                !v
                            } {}
                            (x, z)
                        };


                        let choose_mat: f32 = rng.gen();
                        let center: Vec3 = Vec3 {x, y: Self::small_sphere_radius, z};
                        if choose_mat < 0.45 { // diffuse
                            let albedo: Color3 = Color3 {
                                r: rng.gen::<f32>() * rng.gen::<f32>(),
                                g: rng.gen::<f32>() * rng.gen::<f32>(),
                                b: rng.gen::<f32>() * rng.gen::<f32>()
                            };
                            v.push(MovingHitableGenerator {
                                m: 100.0,
                                k: 0.6,
                                v: 10.0 + (4.0 * rng.gen::<f32>() - 2.0),
                                y: Self::small_sphere_radius,
                                sphere_hitable: Box::new(move |y| Box::new(SphereHitable {
                                    center,
                                    radius: Self::small_sphere_radius,
                                    material: Box::new(LambertMaterial{
                                        albedo
                                    })
                                }))
                            });
                        } else if choose_mat < 0.95 { // metal
                            let albedo: Color3 = Color3 {
                                r: 0.5 * (1.0 + rng.gen::<f32>()),
                                g: 0.5 * (1.0 + rng.gen::<f32>()),
                                b: 0.5 * (1.0 + rng.gen::<f32>())
                            };
                            let f = 0.5 * rng.gen::<f32>();
                            v.push(MovingHitableGenerator {
                                m: 200.0,
                                k: 0.5,
                                v: 10.0 + (4.0 * rng.gen::<f32>() - 2.0),
                                y: Self::small_sphere_radius,
                                sphere_hitable: Box::new(move |y| Box::new(SphereHitable {
                                    center: Vec3 {x, y, z},
                                    radius: Self::small_sphere_radius,
                                    material: Box::new(MetalMaterial {
                                        albedo,
                                        f,
                                    })
                                }))
                            });
                        } else {
                            v.push(MovingHitableGenerator {
                                m: 300.0,
                                k: 0.5,
                                v: 10.0 + (4.0 * rng.gen::<f32>() - 2.0),
                                y: Self::small_sphere_radius,
                                sphere_hitable: Box::new(move |y| Box::new(SphereHitable {
                                    center: Vec3 {x, y, z},
                                    radius: Self::small_sphere_radius,
                                    material: Box::new(DielectricMaterial{ref_idx: 1.5})
                                }))
                            });
                        }

                    }
                    b += 1.2;
                }
                a += 1.2;
            }
            v
        };

        FreeFallAnimation {
            width,
            height,
            dt,
            min_t,
            max_t,
            random_seed,
            t: 0.0,
            look_from_theta: 2.0 * std::f32::consts::PI,
            moving_hitable_generators
        }
    }
}

impl FreeFallAnimation {
    const g: f32 = 9.80665;
    const small_sphere_radius: f32 = 0.2;

    fn update(&mut self) {
        self.camera_update();
        self.physical_update();
    }

    fn camera_update(&mut self) {
        self.look_from_theta += -(2.0 * std::f32::consts::PI / 1200.0)
    }

    fn physical_update(&mut self) {
        self.t += self.dt;
        for hitable_generator in &mut self.moving_hitable_generators {
            let f = - hitable_generator.m * Self::g;
            if hitable_generator.v < 0.0 && hitable_generator.y < Self::small_sphere_radius {
                hitable_generator.v = -hitable_generator.k * hitable_generator.v;
            } else {
                let a = f / hitable_generator.m;
                hitable_generator.v += a * self.dt;
            }
            hitable_generator.y += hitable_generator.v * self.dt;
        }
    }
}

impl Iterator for FreeFallAnimation {
    type Item = Scene;
    // TODO: impl
    fn next(&mut self) -> Option<Self::Item> {

        // Skip
        if (self.t < self.min_t) {
            self.update();
            return self.next();
        }

        let hitable = if self.t > self.max_t {
            None
        } else {
            let camera: Camera = {
                // TODO: Hard code
                let lookfrom: Vec3 = Vec3 {x: 13.0, y: 2.0, z: 3.0};
                let lookat  : Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
                let focus_dist: f32 = 10.0;
                let aperture  : f32  = 0.1;
                Camera {
                    lookfrom,
                    lookat,
                    vup: Vec3 {x: 0.0, y: 1.0, z: 0.0},
                    vfov: 20.0,
                    aspect: self.width as f32 / self.height as f32,
                    aperture,
                    focus_dist
                }
            };


            let hitables: Vec<Box<dyn Hitable + Sync>> = vec![
                // ground
                Box::new(SphereHitable {
                    center: Vec3{x: 0.0, y: -1000.0, z: 0.0},
                    radius: 1000.0,
                    material: Box::new(LambertMaterial{albedo: Color3{r: 0.5, g: 0.5, b: 0.5}})
                }),
                // small balls
                Box::new(ListHitable{hitables: self.moving_hitable_generators.iter().map(|g| (g.sphere_hitable)(g.y)).collect()}),
                Box::new(SphereHitable {
                    center: Vec3{x: 0.0, y: 1.0, z: 0.0},
                    radius: 1.0,
                    material: Box::new(DielectricMaterial{ref_idx: 1.5})
                }),
                Box::new(SphereHitable {
                    center: Vec3{x: -4.0, y: 1.0, z: 0.0},
                    radius: 1.0,
                    material: Box::new(LambertMaterial{albedo: Color3{r: 0.4, g: 0.2, b: 0.1}})
                }),
                Box::new(SphereHitable {
                    center: Vec3{x: 4.0, y: 1.0, z: 0.0},
                    radius: 1.0,
                    material: Box::new(MetalMaterial{albedo: Color3{r: 0.7, g: 0.6, b: 0.5}, f: 0.0})
                })
            ];
            Some(Scene {
                camera,
                hitable: Box::new(ListHitable{ hitables })
            })
        };

        self.update();

        hitable
    }
}
