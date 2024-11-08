use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound, Sound, PlaySoundParams};

struct Bird {
    body: Rect,
    velocity: f32,
}

impl Bird {
    fn fall(&mut self) {
        self.velocity += 0.5;
        self.body.y += self.velocity;
    }
    fn jump(&mut self) {
        self.velocity = -8.0;  
    } 
    fn die(&mut self, pipes: &mut Vec<Pipe>) {
        self.velocity = 0.0;
        self.body.y = screen_height()/2.0;
        *pipes = vec![];
        
    }
}

fn make_bird(x: f32, y: f32, w: f32, h: f32, velocity: f32) -> Bird {
    Bird {
        body: Rect::new(x, y, w, h),
        velocity,
    }
}

#[derive(Clone)]
struct Pipe {
    body_upper: Rect,
    body_lower: Rect,
    scored: bool,
}
impl Pipe {
    fn go(&mut self, score: f32) {
        self.body_lower.x -= 5.0 + score as f32 /5.0;
        self.body_upper.x -= 5.0 + score as f32 /5.0;
    }
    fn edge(&self) -> bool {
        self.body_upper.x < 0.0 - self.body_upper.w
    }
}
fn build_pipe(x: f32, y: f32, w: f32, h: f32) -> Pipe {
    Pipe {
        body_lower: Rect::new(x, y+rand::gen_range(120.0, 250.0)+h, w, screen_height()),
        body_upper: Rect::new(x, y, w, h),
        scored: false,
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird".to_owned(),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main{window_conf}]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut bird = make_bird(100.0, screen_height()/2.0, 60.0, 50.0, 0.0);
    let mut pipes: Vec<Pipe> = vec![];
    let mut i = 1;
    let mut collision_detected = false;
    let background = load_texture("assets/background.png").await.unwrap();
    //background.set_filter(FilterMode::Linear);
    let mut offset = 0.0;

    let flappy = load_texture("assets/flappy.png").await.unwrap();
    let flappy_jump = load_texture("assets/flappy_jump.png").await.unwrap();
    let mut flappy_animation = 100;

    let pipe_texture_upper = load_texture("assets/pipe_upper.png").await.unwrap();
    let pipe_texture_lower = load_texture("assets/pipe_lower.png").await.unwrap();
    let pipe_texture_body = load_texture("assets/pipe_body.png").await.unwrap();

    let flap: Sound = load_sound("assets/flap.ogg").await.unwrap();
    let hit: Sound = load_sound("assets/hit.ogg").await.unwrap();
    let point: Sound = load_sound("assets/point.ogg").await.unwrap();

    let music: Sound = load_sound("assets/music.ogg").await.unwrap();
    play_sound(&music, PlaySoundParams { looped: true, volume: 0.2 });

    let mut score = 0;
    let mut highest_score = 0;

    let mut running = false;
    let mut pressed = false;

    loop {
        clear_background(DARKBLUE);
        
        draw_scrolling_background(&background, 1.0, &mut offset);

        //HANDLE PIPES
        for pipe in &mut pipes {
            if running {
                pipe.go(score as f32);
            }
            
            let mut y_lower = pipe.body_lower.y;
            let mut y_upper = -pipe.body_upper.w;
            //pipe.body_upper.h - pipe.body_upper.w;

            draw_texture_ex(
                &pipe_texture_lower,
                pipe.body_lower.x,
                pipe.body_lower.y,        
                Color::from_rgba(255, 255, 255, 255),
                DrawTextureParams {
                    dest_size: Some(vec2(pipe.body_lower.w, pipe.body_lower.w)),
                    ..Default::default()
                }
            );

            while y_lower < screen_height() {
                y_lower += pipe.body_lower.w - 1.0;
                draw_texture_ex(
                                &pipe_texture_body,
                                pipe.body_lower.x,
                                y_lower,        
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some(vec2(pipe.body_lower.w, pipe.body_lower.w)),
                                    ..Default::default()
                                }
                            );
            }

            while y_upper < pipe.body_upper.h - 2.0*pipe.body_upper.w {
                y_upper += pipe.body_upper.w -1.0;
                draw_texture_ex(
                                &pipe_texture_body,
                                pipe.body_upper.x,
                                y_upper,        
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some(vec2(pipe.body_upper.w, pipe.body_upper.w)),
                                    ..Default::default()
                                }
                            );
            }

            draw_texture_ex(
                &pipe_texture_upper,
                pipe.body_upper.x,
                pipe.body_upper.h - pipe.body_upper.w,        
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(pipe.body_upper.w, pipe.body_upper.w)),
                    ..Default::default()
                }
            );

            if pipe.body_upper.intersect(bird.body).is_some() || pipe.body_lower.intersect(bird.body).is_some() {
                println!("collission detected");
                collision_detected = true;
            }

            //COUNT SCORE
            if bird.body.x > pipe.body_lower.x && !pipe.scored  {
                score += 1;
                pipe.scored = true;
                play_sound(&point, PlaySoundParams { looped: false, volume: 0.1 });
            }

        }
        if collision_detected || bird.body.y + bird.body.h > screen_height() || bird.body.y < 0.0 {
            play_sound(&hit, PlaySoundParams { looped: false, volume: 1.0 });
            bird.die(&mut pipes);
            collision_detected = false;
            score = 0;
            running = false;
        }
        if pipes.len() >= 1 {
            if pipes[pipes.len()-1].body_lower.x < screen_width() - rand::gen_range(300.0, 500.0) {
                pipes.push(build_pipe(screen_width(), 0.0, screen_width()/10.0, rand::gen_range(100.0,screen_height() - 350.0)));
            }
        } else if running{
            pipes.push(build_pipe(screen_width(), 0.0, screen_width()/10.0, rand::gen_range(100.0,screen_height() - 350.0)));
        }
        pipes.retain(|pipe| !pipe.edge());

        //HANDLE BIRD
        if running {
            if i%1 == 0 {
                bird.fall();
            }
        } else {
            draw_text("PRESS SPACE/LMB TO BEGIN", screen_width()/2.0 - 370.0, screen_height()/2.0 - 100.0, 70.0, WHITE);
        }

        if (is_key_pressed(macroquad::input::KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left)) && !pressed{
            bird.jump(); 
            flappy_animation = 0;
            running = true;
            play_sound(&flap, PlaySoundParams { looped: false, volume: 1.0 });
            pressed = true;
        } 
        if is_key_released(macroquad::input::KeyCode::Space) ||  is_mouse_button_pressed(MouseButton::Left){
            pressed = false;
        }
        
        if flappy_animation < 20 {
            draw_texture_ex(
                &flappy_jump,
                bird.body.x, 
                bird.body.y,        
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(bird.body.w, bird.body.h)),
                    ..Default::default()
                }
            );
        }else {
            draw_texture_ex(
                &flappy,
                bird.body.x, 
                bird.body.y,        
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(bird.body.w, bird.body.h)),
                    ..Default::default()
                }
            );
        }

        if score > highest_score {
            highest_score = score;
        }

        draw_text(format!("Highest score: {}", highest_score).as_str(), 30.0, 40.0, 40.0, WHITE);
        draw_text(format!("Current score: {}", score).as_str(), 30.0, 70.0, 40.0, WHITE);

        if running {
            i+=1;
            flappy_animation += 1;
        } else {
            i = 1;
            flappy_animation = 100;
        }
        
        next_frame().await;
    }
}

fn draw_scrolling_background(background: &Texture2D, scroll_speed: f32, offset: &mut f32) {
    *offset += scroll_speed;

    if *offset >= background.width() {
        *offset = 0.0;
    }

    draw_texture_ex(
        background,
         -(*offset), 
        0.0,        
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(background.width(), screen_height())),
            ..Default::default()
        }
    );
    draw_texture_ex(
        background,
        -(*offset) + background.width()-1.0, 
        0.0,        
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(background.width(), screen_height())),
            ..Default::default()
        }
    );
}
