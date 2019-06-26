use crate::ray::Ray;
use crate::hitable::Hitable;
use crate::hitable::HitRecord;

pub struct ListHitable<H: Hitable> {
    pub hitables: std::vec::Vec<H>
}

impl<H: Hitable> Hitable for ListHitable<H> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far: f32 = t_max;
        let mut hit_record_opt: Option<HitRecord> = None;
        for hitable in &self.hitables {
            if let Some(hit_record) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_record_opt = Some(hit_record);
            }
        }
        hit_record_opt
    }
}
