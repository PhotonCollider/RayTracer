use crate::color::write_color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::util::random_in_unit_disk;
use crate::vec3::Vec3;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};

pub struct Camera {
    pub image_width: u32,
    image_height: u32,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    pub sample_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Vec3, // Point camera is looking from
    pub lookat: Vec3,   // Point camera is looking at
    pub vup: Vec3,      // Camera-relative "up" direction
    u: Vec3,
    v: Vec3,
    w: Vec3, // orthonormal basis
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius

    pub part_num_y: u32,
    pub part_num_x: u32,
    part_height: u32,
    part_width: u32,
    thread_limit: u32,

    bar: ProgressBar,
    pub aspect_ratio: f64,

    pub background: Vec3,

    sub_pixel_cnt: u32,
    pub enable_ssaa: bool,
}

impl Camera {
    pub fn default() -> Self {
        Camera {
            image_width: 0,
            image_height: 0,
            camera_center: Vec3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel00_loc: Vec3::zero(),
            sample_per_pixel: 100,
            max_depth: 50,
            vfov: 90.0,
            lookfrom: Vec3::zero(),
            lookat: Vec3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
            defocus_angle: 0.0, // Variation angle of rays through each pixel
            focus_dist: 10.0,   // Distance from camera lookfrom point to plane of perfect focus
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
            part_num_y: 20,
            part_num_x: 20,
            part_height: 0,
            part_width: 0,
            thread_limit: 16,
            bar: ProgressBar::new(1),
            aspect_ratio: 16.0 / 9.0,
            background: Vec3::zero(),
            sub_pixel_cnt: 1,
            enable_ssaa: true,
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        // sub pixel (SSAA)
        self.sub_pixel_cnt = ((self.sample_per_pixel as f64).sqrt() + 0.999).floor() as u32;
        assert!(self.sub_pixel_cnt >= 1);
        println!("sample_per_pixel: {}", self.sample_per_pixel);
        println!("sub_pixel_cnt: {}", self.sub_pixel_cnt);

        // partition
        assert_eq!(self.image_height % self.part_num_y, 0);
        assert_eq!(self.image_width % self.part_num_x, 0);
        self.part_height = self.image_height / self.part_num_y;
        self.part_width = self.image_width / self.part_num_x;

        // ProgressBar
        self.bar = ProgressBar::new(self.image_height as u64 * self.image_width as u64);
        self.bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{msg}  {bar:40.cyan/blue} {pos:>7}/{len:7}  {per_sec}   {eta_precise}"),
        );
        self.bar.set_message("|0 threads outstanding|");

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width =
            viewport_height * (self.image_width as f64) / (self.image_height as f64);
        self.camera_center = self.lookfrom;

        self.w = (self.lookfrom - self.lookat).unit();
        self.u = self.vup.cross(self.w).unit();
        self.v = self.w.cross(self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u;
        let viewport_v = -viewport_height * self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.camera_center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render(&mut self, world: &(impl Hittable + Send + Sync)) -> RgbImage {
        self.initialize();

        // println!("started rendering");

        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        let img_mtx = Arc::new(Mutex::new(&mut img)); // wrap with &mut

        let camera_wrapper1 = Arc::new(self); // Arc<&Camera>，注意内部包装的是 ref
        let camera_wrapper = camera_wrapper1.clone(); // will be moved

        // spawn threads
        crossbeam::thread::scope(move |thread_spawner| {
            let thread_count = Arc::new(AtomicUsize::new(0));
            let thread_number_controller = Arc::new(Condvar::new());
            for y in 0..camera_wrapper.part_num_y {
                for x in 0..camera_wrapper.part_num_x {
                    let ymin = y * camera_wrapper.part_height;
                    let ymax = (y + 1) * camera_wrapper.part_height;
                    let xmin = x * camera_wrapper.part_width;
                    let xmax = (x + 1) * camera_wrapper.part_width;

                    let lock_for_condv = Mutex::new(false);
                    while !(thread_count.load(Ordering::SeqCst)
                        < camera_wrapper.thread_limit as usize)
                    {
                        // outstanding thread number control
                        drop(
                            thread_number_controller
                                .wait(lock_for_condv.lock().unwrap())
                                .unwrap(),
                        );
                    }

                    // move "thread_count++" out of child thread, so that it's sequential with thread number control code
                    thread_count.fetch_add(1, Ordering::SeqCst);
                    camera_wrapper.bar.set_message(format!(
                        "|{} threads outstanding|",
                        thread_count.load(Ordering::SeqCst)
                    )); // set "thread_count" information to progress bar

                    // clone for moving
                    let camera_wrapper = camera_wrapper.clone(); // 每一个子线程需要重新 clone 一个 Arc，相当于引用计数 + 1
                    let img_mtx = img_mtx.clone();
                    let thread_count = thread_count.clone();
                    let thread_number_controller = thread_number_controller.clone();

                    let _ = thread_spawner.spawn(move |_| {
                        camera_wrapper.render_sub(world, ymin, ymax, xmin, xmax, img_mtx);

                        thread_count.fetch_sub(1, Ordering::SeqCst); // subtract first, then notify.
                        camera_wrapper.bar.set_message(format!(
                            "|{} threads outstanding|",
                            thread_count.load(Ordering::SeqCst)
                        ));
                        // NOTIFY
                        thread_number_controller.notify_one();
                    });
                }
            }
        })
        .unwrap();
        std::process::Command::new("clear").status().unwrap();
        camera_wrapper1.bar.finish();
        img
    }

    // ymin..ymax , xmin..xmax
    fn render_sub(
        &self,
        world: &impl Hittable,
        ymin: u32,
        ymax: u32,
        xmin: u32,
        xmax: u32,
        img_mtx: Arc<Mutex<&mut RgbImage>>,
    ) {
        // println!("started thread");
        // Render
        let mut buffer =
            vec![vec![Vec3::zero(); self.image_width as usize]; self.image_height as usize];
        for j in ymin..ymax {
            for i in xmin..xmax {
                if self.enable_ssaa {
                    for sub_y in 0..self.sub_pixel_cnt {
                        for sub_x in 0..self.sub_pixel_cnt {
                            let r = self.get_ray_subpixel(i, j, sub_y, sub_x);
                            buffer[(j - ymin) as usize][(i - xmin) as usize] +=
                                self.ray_color(&r, world, self.max_depth);
                        }
                    }
                } else {
                    for _ in 0..self.sample_per_pixel {
                        let r = self.get_ray(i, j);
                        buffer[(j - ymin) as usize][(i - xmin) as usize] +=
                            self.ray_color(&r, world, self.max_depth);
                    }
                }
                self.bar.inc(1);
            }
        }

        let mut img_guard = img_mtx.lock().unwrap(); // 相当于 lock_guard, 会自动就解锁。
        for j in ymin..ymax {
            for i in xmin..xmax {
                write_color(
                    buffer[(j - ymin) as usize][(i - xmin) as usize]
                        / (self.sample_per_pixel as f64),
                    *img_guard,
                    i as usize,
                    j as usize,
                );
            }
        }
    }

    fn ray_color(&self, r: &Ray, world: &impl Hittable, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::zero();
        }
        let mut rec = HitRecord::new();

        // If the ray hits nothing, return the background color.
        if !world.hit(r, Interval::with_bounds(0.001, f64::INFINITY), &mut rec) {
            return self.background;
        }

        let mut scattered = Ray::default();
        let mut attenuation = Vec3::zero();
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, rec.p);

        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }

        let color_from_scatter =
            attenuation.component_mul(self.ray_color(&scattered, world, depth - 1));

        color_from_emission + color_from_scatter
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let mut rng = rand::thread_rng();

        let pixel_sample = self.pixel00_loc
            + ((i as f64 + rng.gen_range(-0.5..0.5)) * self.pixel_delta_u)
            + ((j as f64 + rng.gen_range(-0.5..0.5)) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction, rng.gen_range(0.0..=1.0))
    }

    fn get_ray_subpixel(&self, i: u32, j: u32, sub_y: u32, sub_x: u32) -> Ray {
        let mut rng = rand::thread_rng();

        let pixel_sample = self.pixel00_loc
            + ((i as f64 + (sub_x * 2 + 1) as f64 / self.sub_pixel_cnt as f64 / 2.0 - 0.5)
                * self.pixel_delta_u)
            + ((j as f64 + (sub_y * 2 + 1) as f64 / self.sub_pixel_cnt as f64 / 2.0 - 0.5)
                * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction, rng.gen_range(0.0..=1.0))
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        return self.camera_center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v);
    }
}
