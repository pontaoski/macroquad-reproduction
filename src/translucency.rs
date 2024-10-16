use macroquad::prelude::*;

#[macroquad::main("Translucency")]
async fn main() {
    let target = render_target(8, 8);
    let mut cam = Camera2D::from_display_rect(Rect::new(0., 0., 8., 8.));
    cam.render_target = Some(target.clone());
    let tile = Texture2D::from_file_with_format(include_bytes!("tile.png"), None);
    let checkerboard = Texture2D::from_file_with_format(include_bytes!("checkerboard.png"), None);

	loop {
		set_camera(&cam);
		clear_background(WHITE);
		draw_texture(&tile, 0., 0., WHITE);
		draw_rectangle(0., 0., 8., 8., Color::new(0., 0., 0., 0.2));

		set_default_camera();
		clear_background(WHITE);
		draw_texture_ex(&checkerboard, 0., 0., WHITE, DrawTextureParams { dest_size: Some(Vec2::new(screen_width(), screen_height())), ..Default::default() });

		draw_texture_ex(&target.texture, 0., 0., WHITE, DrawTextureParams { dest_size: Some(Vec2::new(screen_width(), screen_height())), ..Default::default() });

		next_frame().await;
	}
}
