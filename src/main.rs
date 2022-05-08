use ggez::input::keyboard::{self, KeyCode};
use ggez::{event, graphics};
use ggez::{mint, Context, ContextBuilder, GameResult};
use rand::{self, thread_rng, Rng};

const PADDING: f32 = 40.0;
const PLAYER_SPEED: f32 = 500.0;
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const BALL_SIZE: f32 = 20.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const BALL_SPEED: f32 = 450.0;
const MIDDLE_LINE_WIDTH: f32 = 2.0;

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos: &mut mint::Point2<f32>, key_code: KeyCode, y_dir: f32, ctx: &mut Context) {
    let screen_h = graphics::drawable_size(&ctx).1;
    let dt = ggez::timer::delta(&ctx).as_secs_f32();

    if keyboard::is_key_pressed(&ctx, key_code) {
        pos.y += y_dir * PLAYER_SPEED * dt;
    }

    clamp(
        &mut pos.y,
        RACKET_HEIGHT_HALF,
        screen_h - RACKET_HEIGHT_HALF,
    );
}

fn randomize_vec(vec: &mut mint::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

struct MainState {
    player_1_pos: mint::Point2<f32>,
    player_2_pos: mint::Point2<f32>,
    ball_pos: mint::Point2<f32>,
    ball_vel: mint::Vector2<f32>,
    player_1_score: u32,
    player_2_score: u32,
}

impl MainState {
    pub fn new(ctx: &Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(&ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        let mut ball_vel = mint::Vector2::<f32> { x: 0.0, y: 0.0 };
        randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_pos: mint::Point2::<f32> {
                x: RACKET_WIDTH_HALF + PADDING,
                y: screen_h_half - RACKET_HEIGHT,
            },
            player_2_pos: mint::Point2::<f32> {
                x: screen_w - RACKET_WIDTH_HALF - PADDING,
                y: screen_h_half - RACKET_HEIGHT,
            },
            ball_pos: mint::Point2::<f32> {
                x: screen_w_half,
                y: screen_h_half,
            },
            ball_vel,
            player_1_score: 0,
            player_2_score: 0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let (screen_w, screen_h) = ggez::graphics::drawable_size(&ctx);

        move_racket(&mut self.player_1_pos, KeyCode::W, -1.0, ctx);
        move_racket(&mut self.player_1_pos, KeyCode::S, 1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Up, -1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Down, 1.0, ctx);
        self.ball_pos.x += self.ball_vel.x * dt;
        self.ball_pos.y += self.ball_vel.y * dt;

        if self.ball_pos.x <= 0.0 {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 10;
        }

        if self.ball_pos.x >= screen_w {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 10;
        }
        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        } else if self.ball_pos.y > screen_h - BALL_SIZE_HALF {
            self.ball_pos.y = screen_h - BALL_SIZE_HALF;
            self.ball_vel.y = -self.ball_vel.y.abs();
        }

        let intersects_player_1 = self.ball_pos.x - BALL_SIZE_HALF
            <= self.player_1_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF >= self.player_1_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF <= self.player_1_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF >= self.player_1_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_1 {
            self.ball_vel.x = self.ball_vel.x.abs();
        }

        let intersects_player_2 = self.ball_pos.x - BALL_SIZE_HALF
            <= self.player_2_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF >= self.player_2_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF <= self.player_2_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF >= self.player_2_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_2 {
            self.ball_vel.x = -self.ball_vel.x.abs();
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        let (screen_w, screen_h) = graphics::drawable_size(&ctx);

        let racket_rect = graphics::Rect::new(
            -RACKET_WIDTH_HALF,
            -RACKET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            graphics::Color::YELLOW,
        )?;

        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::Color::RED,
        )?;

        let middle_line_rect =
            graphics::Rect::new(-MIDDLE_LINE_WIDTH * 0.5, 0.0, MIDDLE_LINE_WIDTH, screen_h);
        let middle_line_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            middle_line_rect,
            graphics::Color::WHITE,
        )?;

        let score_text = ggez::graphics::Text::new(format!(
            "{}       {}",
            self.player_1_score, self.player_2_score
        ));

        let screen_w_half = screen_w * 0.5;

        let score_pos = mint::Point2::<f32> {
            x: screen_w_half,
            y: screen_h * 0.1,
        };

        let draw_param = graphics::DrawParam::default();

        let screen_middle_x = graphics::drawable_size(ctx).0 * 0.5;
        graphics::draw(
            ctx,
            &middle_line_mesh,
            draw_param.dest(mint::Point2 {
                x: screen_middle_x,
                y: 0.0,
            }),
        )?;

        graphics::draw(ctx, &racket_mesh, draw_param.dest(self.player_1_pos))?;

        graphics::draw(ctx, &racket_mesh, draw_param.dest(self.player_2_pos))?;
        graphics::draw(ctx, &ball_mesh, draw_param.dest(self.ball_pos))?;

        graphics::draw(
            ctx,
            &score_text,
            draw_param.dest(mint::Point2 {
                x: score_pos.x * 0.89,
                y: score_pos.y,
            }),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let c = ggez::conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("POnG", "obamium3157")
        .default_conf(c)
        .build()
        .unwrap();
    graphics::set_window_title(&ctx, "PONG");
    let state = MainState::new(&ctx);
    event::run(ctx, event_loop, state);
}
