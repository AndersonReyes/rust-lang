use crate::{
    geometry::intersectable::{HitRecord, Intersectable},
    interval::Interval,
    materials::material::{Material, MaterialResult},
    math::{Vector3f, ray::Ray},
};

pub struct Sphere<'a> {
    pub radius: f64,
    pub center: Vector3f,
    pub material: &'a Box<dyn Material>,
}

impl<'a> Sphere<'a> {
    pub fn new(radius: f64, center: Vector3f, material: &'a Box<dyn Material>) -> Self {
        Self {
            radius,
            center,
            material,
        }
    }
}

impl Intersectable for Sphere<'_> {
    fn intersect(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        //        vec3 oc = center - r.origin();
        //auto a = r.direction().length_squared();
        //auto h = dot(r.direction(), oc);
        //auto c = oc.length_squared() - radius*radius;

        //auto discriminant = h*h - a*c;
        //if (discriminant < 0)
        //    return false;

        //auto sqrtd = std::sqrt(discriminant);
        //let oc: Vector3f = self.center - ray.origin;
        //let a: f64 = ray.direction.norm_squared();
        //let h: f64 = ray.direction.dot(&oc);
        //let c: f64 = oc.norm_squared() - self.radius * self.radius;

        //let discriminant: f64 = h * h - a * c;

        //Some(discriminant)
        //    .filter(|d| *d > 0.0)
        //    .and_then(|d| {
        //        let sqrtd = d.sqrt();

        //        // Find the nearest root that lies in the acceptable range.
        //        let mut root = (h - sqrtd) / a;
        //        println!(
        //            "rooot -  = {}, {:#?}, {}",
        //            root,
        //            interval,
        //            interval.surrounds(root)
        //        );

        //        if !interval.surrounds(root) {
        //            root = (h + sqrtd) / a;
        //            println!(
        //                "rooot +  = {}, {:#?}, {}",
        //                root,
        //                interval,
        //                interval.surrounds(root)
        //            );

        //            if !interval.surrounds(root) {
        //                None
        //            } else {
        //                Some(root)
        //            }
        //        } else {
        //            None
        //        }
        //    })
        //    .and_then(|root| {
        //        let intersection_point = ray.at(root);
        //        let time = root;
        //        let normal = (intersection_point - self.center) / self.radius;

        //        Some(HitRecord::new(
        //            intersection_point,
        //            normal,
        //            self.color,
        //            time,
        //            ray,
        //        ))
        //    });

        // vec3 oc = center - r.origin();
        // auto a = dot(r.direction(), r.direction());
        // auto b = -2.0 * dot(r.direction(), oc);
        // auto c = dot(oc, oc) - radius*radius;
        // auto discriminant = b*b - 4*a*c;
        // return (discriminant >= 0);
        //

        let oc: Vector3f = self.center - ray.origin;
        let aa = ray.direction.dot(&ray.direction);
        let bb = -2.0 * ray.direction.dot(&oc);
        let cc = oc.dot(&oc) - self.radius * self.radius;
        let dd = bb * bb - 4.0 * aa * cc;

        let intersection_point = ray.at(dd);
        let time = dd;
        let normal = (intersection_point - self.center) / self.radius;

        Some(dd).filter(|root| *root >= 0.0).map(|_| {
            let mat_result: MaterialResult = self
                .material
                .get_color(ray, &normal, &intersection_point)
                .expect("Failed to get color from material for sphere");

            HitRecord::new(
                intersection_point,
                normal,
                mat_result.attenuation,
                time,
                ray,
            )
        })
    }
}
