pub struct AnimationEffect {
    pub name: &'static str,
    pub hex: u8
}
pub const ANIMATIONS: [AnimationEffect; 20] = [
    AnimationEffect { name: "Swim", hex: 0x00 },
    AnimationEffect { name: "Gossip / Sea", hex: 0x01 },
    AnimationEffect { name: "Rainbow wheel", hex: 0x02 },
    AnimationEffect { name: "Ripple diffusion", hex: 0x03 },
    AnimationEffect { name: "Retinue scanning", hex: 0x04 },
    AnimationEffect { name: "Crossfire", hex: 0x05 },
    AnimationEffect { name: "Respire", hex: 0x06 },
    AnimationEffect { name: "Blossoming", hex: 0x07 },
    AnimationEffect { name: "Shadow follows", hex: 0x08 },
    AnimationEffect { name: "Comet tail-tracking", hex: 0x09 },
    AnimationEffect { name: "Rotating windmill", hex: 0x0A },
    AnimationEffect { name: "Fixed on", hex: 0x0B },
    AnimationEffect { name: "Stars twinkle", hex: 0x0C },
    AnimationEffect { name: "Neon stream", hex: 0x0D },
    AnimationEffect { name: "Instant", hex: 0x0E },
    AnimationEffect { name: "Progressive scanning", hex: 0x0F },
    AnimationEffect { name: "Collision", hex: 0x10 },
    AnimationEffect { name: "Rotating storm", hex: 0x11 },
    AnimationEffect { name: "Random flying", hex: 0x12 },
    AnimationEffect { name: "Colorful waterfalls", hex: 0x13 },
];

impl AnimationEffect {
    pub fn find_name(name: &str) -> Option<&AnimationEffect> {
        for animation in ANIMATIONS.iter() {
            if animation.name.eq_ignore_ascii_case(name) {
                return Some(animation);
            }
        }
        None
    }
    pub fn find_id(id: usize) -> Option<&'static AnimationEffect> {
        if id < ANIMATIONS.len() {
            Some(&ANIMATIONS[id as usize])
        } else {
            None
        }
    }
}