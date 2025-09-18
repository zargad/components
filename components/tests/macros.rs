use components::channels::Channel;
use components::channels::DuelChannel;
use components::Channels;

#[test]
fn test_derive_channels_get() {
    let channel = get_test_channel();
    let color: Rgb = channel.get();
    assert_eq!(color, channel.color);
}

#[test]
fn test_derive_channels_set() {
    let channel = get_test_channel();
    let new_position = (-5, 20);
    let channel = channel.set(new_position);
    assert_eq!(channel.position, new_position);
}

#[test]
fn test_derive_duel_channels_get() {
    let channel = get_test_channel();
    let color: Rgb = channel.duel_get();
    assert_eq!(color, channel.color);
}

#[test]
fn test_derive_duel_channels_set() {
    let channel = get_test_channel();
    let new_position = (-5, 20);
    let channel = channel.duel_set(new_position);
    assert_eq!(channel.position, new_position);
}

const fn get_test_channel() -> GraphicsChannel {
    let color = Rgb {
        red: 255,
        green: 0,
        blue: 0,
    };
    GraphicsChannel {
        position: (10, 10),
        color,
    }
}

#[derive(Debug, Clone, Copy, Channels)]
struct GraphicsChannel {
    position: (isize, isize),
    color: Rgb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}
