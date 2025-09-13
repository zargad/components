use components::channels::Channel;
use components::Channels;

#[test]
fn test_derive_channels_get() {
    let channel = get_test_channel();
    let color: Rgb = <GraphicsChannel as Channel<Rgb, (isize, isize)>>::get(&channel);
    assert_eq!(color, channel.color);
}

#[test]
fn test_derive_channels_set() {
    let channel = get_test_channel();
    let new_position = (-5, 20);
    let channel = <GraphicsChannel as Channel<Rgb, (isize, isize)>>::set(&channel, new_position);
    assert_eq!(channel.position, new_position);
}

fn get_test_channel() -> GraphicsChannel {
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
