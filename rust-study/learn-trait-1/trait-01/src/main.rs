trait Sport {
    
    fn play (&self);
    fn play_mut (&mut self);
    fn play_own(self);
    fn play_some() -> Self;

}
struct FootBall;

impl Sport for FootBall {
    fn play (&self) {}
    fn play_mut (&mut self) {
        
    }
    
    fn play_own(self) {
        
    }
    fn play_some() -> Self {
       Self
    }
}


fn main() {
    let mut f = FootBall;
    f.play();
    f.play_mut();
    f.play_own();
    let _g = FootBall::play_some();
    let _g = <FootBall as Sport>::play_some();

    println!("Hello, world!");
}
