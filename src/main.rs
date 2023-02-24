use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::event;
use ggez::input::keyboard::{self, KeyCode};
use rand::{self, thread_rng,Rng};

const linew:f32 = 2.0;
const padding:f32 = 40.0;
const hracket:f32 = 100.0;
const wracket:f32 = 20.0;
const halfhracket:f32 = hracket / 2.0;
const halfwracket:f32 = wracket / 2.0;
const ball:f32 = 30.0;
const halfball:f32 = ball / 2.0;
const playerspeed:f32 = 600.0;
const ballspeed:f32 = 350.0;

fn clamp(value: &mut f32, low:f32, high:f32){
    if *value<low{
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos:&mut na::Point2<f32>, keycode: KeyCode, y_dir:f32,ctx:&mut Context){
    let screenh = graphics::drawable_size(ctx).1;
    let dt = ggez::timer::delta(ctx).as_secs_f32();
    if keyboard::is_key_pressed(ctx, keycode){
        pos.y += y_dir * playerspeed * dt;
    }
    clamp(&mut pos.y,halfhracket,screenh-halfhracket);
}

fn random(vec: &mut na::Vector2<f32>,x:f32,y:f32){
    let mut rng = thread_rng();

    vec.x = match rng.gen_bool(0.5){
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5){
        true => y,
        false => -y,
    };
}

struct MainState{
    player1pos: na::Point2<f32>,
    player2pos: na::Point2<f32>,
    ballpos: na::Point2<f32>,
    ballvel: na::Vector2<f32>,
    score1:i32,
    score2:i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screenw,screenh)= graphics::drawable_size(ctx);
        let (halfscreenw,halfscreenh) = (screenw*0.5,screenh/2.0);

        let mut ballvel = na::Vector2::new(0.0,0.0);
        random(&mut ballvel, ballspeed, ballspeed);

        MainState{
            player1pos : na::Point2::new(halfwracket + padding, halfscreenh),
            player2pos : na::Point2::new(screenw-halfwracket - padding, halfscreenh),
            ballpos: na::Point2::new(halfscreenw,halfscreenh),
            ballvel,
            score1:0,
            score2:0,
        }
    }
}

impl event::EventHandler for MainState{
    fn update(&mut self,ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let (screenw,screenh)=graphics::drawable_size(ctx);

        move_racket(&mut self.player1pos,KeyCode::W,-1.0,ctx);
        move_racket(&mut self.player1pos,KeyCode::S,1.0,ctx);
        move_racket(&mut self.player2pos,KeyCode::Up,-1.0,ctx);
        move_racket(&mut self.player2pos,KeyCode::Down,1.0,ctx);

        // if keyboard::is_key_pressed(ctx,KeyCode::W){
        //     self.player1pos.y -= playerspeed*dt;
        // }
        // if keyboard::is_key_pressed(ctx,KeyCode::S){
        //     self.player1pos.y += playerspeed*dt;
        // }
        // clamp(&mut self.player1pos.y,halfhracket,screenh-halfhracket);

        // if keyboard::is_key_pressed(ctx,KeyCode::Up){
        //     self.player2pos.y -= playerspeed*dt;
        // }
        // if keyboard::is_key_pressed(ctx,KeyCode::Down){
        //     self.player2pos.y += playerspeed*dt;
        // }
        // clamp(&mut self.player2pos.y,halfhracket,screenh-halfhracket);

        self.ballpos += self.ballvel*dt;

        if self.ballpos.x < 0.0 {
            self.ballpos.x = screenw / 2.0;
            self.ballpos.y = screenh / 2.0;
            random(&mut self.ballvel, ballspeed, ballspeed);
            self.score2 +=1;
        }
        if self.ballpos.x > screenw {
            self.ballpos.x = screenw / 2.0;
            self.ballpos.y = screenh / 2.0;
            random(&mut self.ballvel, ballspeed, ballspeed);
            self.score1 +=1;
        }

        if self.ballpos.y < halfball {
            self.ballpos.y = halfball;
            self.ballvel.y = self.ballvel.y.abs();
        }else if self.ballpos.y > screenh - halfball {
            self.ballpos.y = screenh - halfball;
            self.ballvel.y = -self.ballvel.y.abs();
        }

        let intersects1 = 
        self.ballpos.x - halfball < self.player1pos.x + halfwracket &&
        self.ballpos.x + halfball > self.player1pos.x - halfwracket &&
        self.ballpos.y - halfball < self.player1pos.y + halfhracket &&
        self.ballpos.y + halfball > self.player1pos.y - halfhracket;

        let intersects2 = 
        self.ballpos.x - halfball < self.player2pos.x + halfwracket &&
        self.ballpos.x + halfball > self.player2pos.x - halfwracket &&
        self.ballpos.y - halfball < self.player2pos.y + halfhracket &&
        self.ballpos.y + halfball > self.player2pos.y - halfhracket;

        if intersects1{
            self.ballvel.x = self.ballvel.x.abs();
        }

        if intersects2{
            self.ballvel.x = -self.ballvel.x.abs();
        }

        Ok(())
    }

    fn draw(&mut self,ctx: &mut Context) -> GameResult{
        graphics::clear(ctx, graphics::BLACK);
        
        let racket_rect = graphics::Rect::new(-halfwracket, -halfhracket, wracket, hracket);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, graphics::WHITE)?;

        let ball_rect = graphics::Rect::new(-halfball, -halfball, ball, ball);
        let ball_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball_rect, graphics::WHITE)?;

        let screenh = graphics::drawable_size(ctx).1;
        let middlerect = graphics::Rect::new(-linew / 2.0, 0.0, linew, screenh);
        let middlemesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), middlerect, graphics::WHITE)?;
        let screenmidx = graphics::drawable_size(ctx).0/2.0;

        let mut draw_param = graphics::DrawParam::default();

        draw_param.dest = [screenmidx, 0.0].into();
        graphics::draw(ctx,&middlemesh,draw_param)?;

        draw_param.dest = self.player1pos.into();
        graphics::draw(ctx,&racket_mesh,draw_param)?;
        
        draw_param.dest = self.player2pos.into();
        graphics::draw(ctx,&racket_mesh,draw_param)?;

        draw_param.dest = self.ballpos.into();
        graphics::draw(ctx,&ball_mesh,draw_param)?;

        let scoretext = graphics::Text::new(format!("{}         {}",self.score1,self.score2));
        let screenw=graphics::drawable_size(ctx).0;
        let halfscreenw = screenw / 2.0;

        let mut scorepos = na::Point2::new(halfscreenw,40.0);
        let (scoretextw,scoretexth) = scoretext.dimensions(ctx);
        scorepos -= na::Vector2::new(scoretextw as f32 / 2.0 , scoretexth as f32 / 2.0);
        draw_param.dest = scorepos.into();

        graphics::draw(ctx,&scoretext,draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Snake_0","TanTan");
    let(ctx,event_loop)=&mut cb.build()?;
    graphics::set_window_title(ctx,"SNAKE");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop,&mut state);
    Ok(())
}