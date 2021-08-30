use macroquad::prelude::*;
use macroquad::models::Vertex;
use macroquad::ui;
use macroquad::audio;

const CAM_SPEED: f32 = 1.5;
const FLOOR_SIZE: f32 = 10.;
const WALL_HEIGHT: f32 = 20.;
const WINDOW_OFFSET: f32 = 0.04;
const WINDOW_WIDTH: f32 = FLOOR_SIZE*6./10.;
const DOOR_WIDTH: f32 = FLOOR_SIZE*5./10.;
const STAGE_DURATION: f64 = 10.;
const INIT_STAGE_DURATION: f64 = 5.;
const STAGE_INTERVAL: f64 = 5.;
const STAGE_COUNT: u16 = 3;
const CHAT_HOLD: f64 = 1.0;
const INIT_STAGE: u16 = 0;
const EYE_SPEED: f32 = 250.;
//const CHAT_SIZE_X: f32 = width-width/5.;
//const CHAT_SIZE_Y: f32 = height-height*5./6.;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Ding Dong!"),
        window_width: 1000,
        window_height: 750,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;

    let mut front;
    let mut right;
    let mut up;
    let position = vec3(-25., 5., 0.);

    set_pc_assets_folder("assets");

    // Load Assets
    let floor = Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../assets/floor2.png"), Some(ImageFormat::Png)));
    let wall = Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../assets/wall2.png"), Some(ImageFormat::Png)));
    let window = Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../assets/window.png"), Some(ImageFormat::Png)));
    let door = Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../assets/door.png"), Some(ImageFormat::Png)));
    let doorbell = audio::load_sound_from_bytes(include_bytes!("../assets/doorbell.wav")).await.unwrap();
    let sleep = audio::load_sound_from_bytes(include_bytes!("../assets/sleep.wav")).await.unwrap();
    let button_yes = Image::from_file_with_format(include_bytes!("../assets/yes.png"), Some(ImageFormat::Png));
    let button_yes_hov = Image::from_file_with_format(include_bytes!("../assets/yes_hov.png"), Some(ImageFormat::Png));
    let button_yes_press = Image::from_file_with_format(include_bytes!("../assets/yes_press.png"), Some(ImageFormat::Png));
    let button_no = Image::from_file_with_format(include_bytes!("../assets/no.png"), Some(ImageFormat::Png));
    let button_no_hov = Image::from_file_with_format(include_bytes!("../assets/no_hov.png"), Some(ImageFormat::Png));
    let button_no_press = Image::from_file_with_format(include_bytes!("../assets/no_press.png"), Some(ImageFormat::Png));
    let window_style = Image::from_file_with_format(include_bytes!("../assets/window_style.png"), Some(ImageFormat::Png));
    let window_blank = Image::from_file_with_format(include_bytes!("../assets/window_blank.png"), Some(ImageFormat::Png));
    let window_blank2 = Image::from_file_with_format(include_bytes!("../assets/window_blank.png"), Some(ImageFormat::Png));

    let sphere = Texture2D::from_image(&Image::from_file_with_format(include_bytes!("../assets/sphere.png"), Some(ImageFormat::Png)));

    // Create meshes
    // Floor
    let v1 = Vertex {position: vec3(-FLOOR_SIZE*3., 0., FLOOR_SIZE*3.), uv: vec2(0., 0.), color: WHITE,};
    let v2 = Vertex {position: vec3(FLOOR_SIZE*3., 0., FLOOR_SIZE*3.), uv: vec2(1., 0.), color: WHITE,};
    let v3 = Vertex {position: vec3(FLOOR_SIZE*3., 0., -FLOOR_SIZE*3.), uv: vec2(1., 1.), color: WHITE,};
    let v4 = Vertex {position: vec3(-FLOOR_SIZE*3., 0., -FLOOR_SIZE*3.), uv: vec2(0., 1.), color: WHITE,};
    let indices = vec![0,1,2,0,2,3];
    let floor = Mesh {
        vertices: vec![v1, v2, v3, v4],
        indices: indices,
        texture: Some(floor),
    };
    // Walls
    let v1 = Vertex {position: vec3(-FLOOR_SIZE*3., 0., FLOOR_SIZE*3.), uv: vec2(0., 0.), color: WHITE,};
    let v2 = Vertex {position: vec3(FLOOR_SIZE*3., 0., FLOOR_SIZE*3.), uv: vec2(1., 0.), color: WHITE,};
    let v3 = Vertex {position: vec3(FLOOR_SIZE*3., WALL_HEIGHT, FLOOR_SIZE*3.), uv: vec2(1., 1.), color: WHITE,};
    let v4 = Vertex {position: vec3(-FLOOR_SIZE*3., WALL_HEIGHT, FLOOR_SIZE*3.), uv: vec2(0., 1.), color: WHITE,};
    let indices = vec![0,1,2,0,2,3];
    let wall_1 = Mesh {
        vertices: vec![v1, v2, v3, v4],
        indices: indices,
        texture: Some(wall),
    };
    let v1 = Vertex {position: vec3(FLOOR_SIZE*3., 0., FLOOR_SIZE*3.), uv: vec2(0., 0.), color: WHITE,};
    let v2 = Vertex {position: vec3(FLOOR_SIZE*3., 0., -FLOOR_SIZE*3.), uv: vec2(1., 0.), color: WHITE,};
    let v3 = Vertex {position: vec3(FLOOR_SIZE*3., WALL_HEIGHT, -FLOOR_SIZE*3.), uv: vec2(1., 1.), color: WHITE,};
    let v4 = Vertex {position: vec3(FLOOR_SIZE*3., WALL_HEIGHT, FLOOR_SIZE*3.), uv: vec2(0., 1.), color: WHITE,};
    let indices = vec![0,1,2,0,2,3];
    let wall_2 = Mesh {
        vertices: vec![v1, v2, v3, v4],
        indices: indices,
        texture: Some(wall),
    };
    let v1 = Vertex {position: vec3(FLOOR_SIZE*3., 0., -FLOOR_SIZE*3.), uv: vec2(0., 0.), color: WHITE,};
    let v2 = Vertex {position: vec3(-FLOOR_SIZE*3., 0., -FLOOR_SIZE*3.), uv: vec2(1., 0.), color: WHITE,};
    let v3 = Vertex {position: vec3(-FLOOR_SIZE*3., WALL_HEIGHT, -FLOOR_SIZE*3.), uv: vec2(1., 1.), color: WHITE,};
    let v4 = Vertex {position: vec3(FLOOR_SIZE*3., WALL_HEIGHT, -FLOOR_SIZE*3.), uv: vec2(0., 1.), color: WHITE,};
    let indices = vec![0,1,2,0,2,3];
    let wall_3 = Mesh {
        vertices: vec![v1, v2, v3, v4],
        indices: indices,
        texture: Some(wall),
    };
    let v1 = Vertex {position: vec3(-FLOOR_SIZE*3., 0., -FLOOR_SIZE*3.), uv: vec2(0., 0.), color: WHITE,};
    let v2 = Vertex {position: vec3(-FLOOR_SIZE*3., 0., FLOOR_SIZE*3.), uv: vec2(1., 0.), color: WHITE,};
    let v3 = Vertex {position: vec3(-FLOOR_SIZE*3., WALL_HEIGHT, FLOOR_SIZE*3.), uv: vec2(1., 1.), color: WHITE,};
    let v4 = Vertex {position: vec3(-FLOOR_SIZE*3., WALL_HEIGHT, -FLOOR_SIZE*3.), uv: vec2(0., 1.), color: WHITE,};
    let indices = vec![0,1,2,0,2,3];
    let wall_4 = Mesh {
        vertices: vec![v1, v2, v3, v4],
        indices: indices,
        texture: Some(wall),
    };
    // Ceiling
    let v1 = Vertex {position: vec3(-FLOOR_SIZE*3., WALL_HEIGHT, FLOOR_SIZE*3.), uv: vec2(0., 0.), color: WHITE,};
    let v2 = Vertex {position: vec3(FLOOR_SIZE*3., WALL_HEIGHT, FLOOR_SIZE*3.), uv: vec2(1., 0.), color: WHITE,};
    let v3 = Vertex {position: vec3(FLOOR_SIZE*3., WALL_HEIGHT, -FLOOR_SIZE*3.), uv: vec2(1., 1.), color: WHITE,};
    let v4 = Vertex {position: vec3(-FLOOR_SIZE*3., WALL_HEIGHT, -FLOOR_SIZE*3.), uv: vec2(0., 1.), color: WHITE,};
    let indices = vec![0,1,2,0,2,3];
    let ceiling = Mesh {
        vertices: vec![v1, v2, v3, v4],
        indices: indices,
        texture: Some(wall),
    };
    // Windows
    let v1 = Vertex {position: vec3(-WINDOW_WIDTH, 5., FLOOR_SIZE*3.-WINDOW_OFFSET), uv: vec2(0., 0.), color: WHITE,};
    let v2 = Vertex {position: vec3(WINDOW_WIDTH, 5., FLOOR_SIZE*3.-WINDOW_OFFSET), uv: vec2(1., 0.), color: WHITE,};
    let v3 = Vertex {position: vec3(WINDOW_WIDTH, 15., FLOOR_SIZE*3.-WINDOW_OFFSET), uv: vec2(1., 1.), color: WHITE,};
    let v4 = Vertex {position: vec3(-WINDOW_WIDTH, 15., FLOOR_SIZE*3.-WINDOW_OFFSET), uv: vec2(0., 1.), color: WHITE,};
    let indices = vec![0,1,2,0,2,3];
    let window_1 = Mesh {
        vertices: vec![v1, v2, v3, v4],
        indices: indices,
        texture: Some(window),
    };
    let v1 = Vertex {position: vec3(-WINDOW_WIDTH, 5., -FLOOR_SIZE*3.+WINDOW_OFFSET), uv: vec2(0., 0.), color: WHITE,};
    let v2 = Vertex {position: vec3(WINDOW_WIDTH, 5., -FLOOR_SIZE*3.+WINDOW_OFFSET), uv: vec2(1., 0.), color: WHITE,};
    let v3 = Vertex {position: vec3(WINDOW_WIDTH, 15., -FLOOR_SIZE*3.+WINDOW_OFFSET), uv: vec2(1., 1.), color: WHITE,};
    let v4 = Vertex {position: vec3(-WINDOW_WIDTH, 15., -FLOOR_SIZE*3.+WINDOW_OFFSET), uv: vec2(0., 1.), color: WHITE,};
    let indices = vec![0,1,2,0,2,3];
    let window_2 = Mesh {
        vertices: vec![v1, v2, v3, v4],
        indices: indices,
        texture: Some(window),
    };
    // Door
    let v1 = Vertex {position: vec3(FLOOR_SIZE*3.-WINDOW_OFFSET, 0., DOOR_WIDTH), uv: vec2(0., 0.), color: WHITE,};
    let v2 = Vertex {position: vec3(FLOOR_SIZE*3.-WINDOW_OFFSET, 0., -DOOR_WIDTH), uv: vec2(1., 0.), color: WHITE,};
    let v3 = Vertex {position: vec3(FLOOR_SIZE*3.-WINDOW_OFFSET, 13., -DOOR_WIDTH), uv: vec2(1., 1.), color: WHITE,};
    let v4 = Vertex {position: vec3(FLOOR_SIZE*3.-WINDOW_OFFSET, 13., DOOR_WIDTH), uv: vec2(0., 1.), color: WHITE,};
    let indices = vec![0,1,2,0,2,3];
    let door = Mesh {
        vertices: vec![v1, v2, v3, v4],
        indices: indices,
        texture: Some(door),
    };

    let skin = {
        let label_style = ui::root_ui()
            .style_builder()
            .text_color(WHITE)
            .font_size(20)
            .build();

        let window_style = ui::root_ui()
            .style_builder()
            .background(window_style)
            .background_margin(RectOffset::new(15., 40., 0., 0.))
            .build();

        ui::Skin {
            label_style,
            window_style,
            ..ui::root_ui().default_skin()
        }
    };
    
    let yes_skin = {
        let window_style = ui::root_ui()
            .style_builder()
            .background(window_blank)
            .build();

        let button_style = ui::root_ui()
            .style_builder()
            .background(button_yes)
            .background_hovered(button_yes_hov)
            .background_clicked(button_yes_press)
            .build();
        
        ui::Skin {
            window_style,
            button_style,
            ..ui::root_ui().default_skin()
        }
    };

    let no_skin = {
        let window_style = ui::root_ui()
            .style_builder()
            .background(window_blank2)
            .build();

        let button_style = ui::root_ui()
            .style_builder()
            .background(button_no)
            .background_hovered(button_no_hov)
            .background_clicked(button_no_press)
            .build();
        
        ui::Skin {
            window_style,
            button_style,
            ..ui::root_ui().default_skin()
        }
    };

    let mut stage_marker: f64 = STAGE_DURATION + INIT_STAGE_DURATION;
    let mut stage_num: u16 = INIT_STAGE;
    //let mut sub_stage_num: u16 = 1;
    let mut stage_changed_at: f64 = 0.;
    let mut stage_type: bool = false;
    let mut eye_time: f32 = 0.;

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            yaw -= CAM_SPEED*delta;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            yaw += CAM_SPEED*delta;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            pitch += CAM_SPEED*delta;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            pitch -= CAM_SPEED*delta;
        }

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };
        yaw = if yaw > 2. { 2. } else { yaw };
        yaw = if yaw < -2. { -2. } else { yaw };

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        ).normalize();
        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });
 
        // Draw wall
        draw_mesh(&floor);
        draw_mesh(&wall_1);
        draw_mesh(&wall_2);
        draw_mesh(&wall_3);
        draw_mesh(&wall_4);
        draw_mesh(&ceiling);
        draw_mesh(&window_1);
        draw_mesh(&window_2);
        draw_mesh(&door);
        draw_sphere(vec3(0., 3., 0.), 2.5, sphere, WHITE);

        let time = get_time();

        
        if stage_num == 0 && time > INIT_STAGE_DURATION {
            stage_num += 1;
            stage_type = true;
            audio::play_sound(doorbell, audio::PlaySoundParams{looped: false, volume: 0.8});
        }
        else if time > stage_marker {
            eye_time = 0.;
            stage_num += 1;
            if stage_num < STAGE_COUNT*2 {
                stage_changed_at = time;
                //println!("Stage changed at: {}", stage_changed_at);
            }
            if stage_num % 2 == 0 {
                stage_marker += STAGE_DURATION/2.;
                stage_type = false;
                audio::play_sound(sleep, audio::PlaySoundParams{looped: false, volume: 0.5})
            }
            else {
                stage_marker += STAGE_DURATION;
                stage_type = true;
                audio::play_sound(doorbell, audio::PlaySoundParams{looped: false, volume: 0.8});
            }
        }
        
        let mut label = String::new();
        let mut comment = String::new();
        let stage_time: f64 = stage_marker-time;
        match stage_num {
            1 => {
                let stage_cut = (STAGE_DURATION-CHAT_HOLD)/3.;
                label = "Mailman".to_string();
                if stage_time > stage_cut*2. {
                    comment = "Package for delivery! It needs a signature, could you open the door?".to_string();
                }
                else if stage_time > stage_cut {
                    comment = "Hello?".to_string();
                }
                else { 
                    comment = "Ok well, you'll have to pick it up from the post office.".to_string();
                }
            }
            3 => {
                let stage_cut = (STAGE_DURATION-CHAT_HOLD)/4.;
                label = "Dog".to_string();
                if stage_time > stage_cut*3. {
                    comment = "*Woof!*".to_string();
                }
                else if stage_time > stage_cut*2. {
                    comment = "*Bark.*".to_string();
                }
                else if stage_time > stage_cut {
                    comment = "*Arf...*".to_string();
                }
                else { 
                    comment = "*Whine*".to_string();
                }
            },
            5 => {
                let stage_cut = (STAGE_DURATION-CHAT_HOLD)/6.;
                label = "Female Scout".to_string();
                if stage_time > stage_cut*5. {
                    comment = "Hi this is Amber from Troop 1295, buy my cookies!".to_string();
                }
                else if stage_time > stage_cut*4. {
                    comment = "We have peanut butter chocolate, oatmeal, mint chocolate, and more!".to_string();
                }
                else if stage_time > stage_cut*3. {
                    comment = "I only need to sell 50 more crates full won't you buy some?".to_string();
                }
                else if stage_time > stage_cut*2. {
                    comment = "Do you not like cookies?".to_string();
                }
                else if stage_time > stage_cut {
                    comment = "Hello? I know you can hear me!".to_string();
                }
                else {
                    comment = "I hate you!".to_string();
                }
            }
            _ => {},
        }

        let width = screen_width();
        let height = screen_height();
        if stage_num < STAGE_COUNT*2 {
            if stage_type {
                if time - stage_changed_at > CHAT_HOLD{
                    let position_x = width/2.-(width-width/5.)/2.;
                    let position_y = height-height/4.;
                    let size_x = width-width/5.;
                    let size_y = height-height*5./6.;
    
                    ui::root_ui().push_skin(&skin);
                    let w1 = ui::widgets::Window::new(1, vec2(position_x, position_y), vec2(size_x, size_y))
                        .titlebar(false)
                        .movable(false)
                        .enabled(true);
                    w1.ui(&mut *ui::root_ui(), |ui| {
                        let len: u16 = label.chars().count() as u16;
                        let mut s = String::new();
                        for i in 1..len {
                            s += "_";
                        }
                        ui.label(Vec2::new(0., 15.), &label);
                        ui.label(Vec2::new(5., 25.), &s);
                        ui.label(Vec2::new(0., 50.), &comment);
                    });
                    ui::root_ui().pop_skin();
    
                    let icon_size:f32  = width/20.;
    
                    ui::root_ui().push_skin(&yes_skin);
                    let w2 = ui::widgets::Window::new(2, vec2(position_x+size_x-60., position_y+10.), vec2(icon_size, icon_size))
                        .titlebar(false)
                        .movable(false)
                        .enabled(true);
                    w2.ui(&mut *ui::root_ui(), |ui| {
                        let b1 = ui::widgets::Button::new("").position(vec2(0.0, 0.0)).size(vec2(icon_size, icon_size));
                        if b1.ui(ui) {
                            stage_marker -= stage_time;
                        }
                    });
                    ui::root_ui().pop_skin();
    
                    ui::root_ui().push_skin(&no_skin);
                    let w2 = ui::widgets::Window::new(3, vec2(position_x+size_x-60., position_y+size_y-10.-icon_size), vec2(icon_size, icon_size))
                        .titlebar(false)
                        .movable(false)
                        .enabled(true);
                    w2.ui(&mut *ui::root_ui(), |ui| {
                        let b2 = ui::widgets::Button::new("").position(vec2(0.0, 0.0)).size(vec2(icon_size, icon_size));
                        if b2.ui(ui) {
                            stage_marker -= stage_time;
                        }
                    });
                    ui::root_ui().pop_skin();

                    if stage_num > 1 {
                        set_default_camera();
                        if 0.-eye_time > -height/2. {
                            eye_time += delta*EYE_SPEED;
                        }
                        if eye_time > height/2. {
                            eye_time = height/2.;
                        }
                        draw_rectangle(0., 0.-eye_time, width, height/2., BLACK);
                        draw_rectangle(0., height/2.+eye_time, width, height/2., BLACK);
                    }
                }
                else {
                    set_default_camera();
                    draw_rectangle(0., 0.-eye_time, width, height/2., BLACK);
                    draw_rectangle(0., height/2.+eye_time, width, height/2., BLACK);
                }
            }
            else {
                set_default_camera();
                if stage_num > 0 {
                    if -height/2.+eye_time < 0. {
                        eye_time += delta*EYE_SPEED/2.;
                    }
                    if eye_time > height/2. {
                        eye_time = height/2.;
                    }
                    draw_rectangle(0., -height/2.+eye_time, width, height/2., BLACK);
                    draw_rectangle(0., height-eye_time, width, height/2., BLACK);
                }
                else {
                    if time - 3.0 > 0. {
                        if 0.-eye_time > -height/2. {
                            eye_time += delta*EYE_SPEED;
                        }
                        if eye_time > height/2. {
                            eye_time = height/2.;
                        }
                        draw_rectangle(0., 0.-eye_time, width, height/2., BLACK);
                        draw_rectangle(0., height/2.+eye_time, width, height/2., BLACK);
                    }
                    else {
                        draw_rectangle(0., 0.-eye_time, width, height/2., BLACK);
                        draw_rectangle(0., height/2.+eye_time, width, height/2., BLACK);
                        draw_text("Ding Dong!", width/2.-50., height/2.-10., 24., WHITE);
                    }
                }
            }
        }
        next_frame().await;
    }
}
