fn main(
    a: bool,
    b: ([bool; 2], [bool; 2], str<2>, (u16, u16), u16),
    c: ((Field, bool, bool), (u16, str<2>)),
) -> pub u16 {
    b.4
        % if c.0.2 {
            b.3.0
        } else {
            if (!a) {
                37995
            } else {
                (b.3.1 % 64485)
            }
        }
}
