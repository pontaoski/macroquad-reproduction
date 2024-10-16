use macroquad::prelude::*;

#[macroquad::main("Well")]
async fn main() {
    let well_width = 20. as f32;
    let well_height = 20. as f32;
    let line_thickness = 1. / 8.;
    let wall = Color::new(0.77625, 0.96804, 1.00513, 0.2);

	loop {
		set_default_camera();
		clear_background(Color::new(0.1, 0.1, 0.5, 1.));

        set_camera(&Camera3D {
            position: vec3(-35., 0., 0.),
            up: vec3(0., -1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_affine_parallelogram(
            Vec3::new(1., well_width / 2., well_height as f32 / -2.),
            -(well_height as f32) * Vec3::Y,
            well_width as f32 * Vec3::Z,
            None,
            Color::new(0., 0., 0., 0.2),
        );

        draw_affine_parallelogram(
            Vec3::new(0., well_height / 2. + line_thickness, well_width / -2.),
            Vec3::new(1., -line_thickness, 0.),
            Vec3::new(0., 0., well_width),
            None,
            wall,
        );

        draw_affine_parallelogram(
            Vec3::new(0., well_height / 2. + line_thickness, well_width / 2.),
            Vec3::new(0., -(well_height + line_thickness), 0.),
            Vec3::new(1., 0., line_thickness),
            None,
            wall,
        );

        draw_affine_parallelogram(
            Vec3::new(
                0.,
                well_height / 2. + line_thickness,
                well_width / -2. - line_thickness,
            ),
            Vec3::new(0., -(well_height + line_thickness), 0.),
            Vec3::new(1., 0., line_thickness),
            None,
            wall,
        );

		next_frame().await;
	}
}
