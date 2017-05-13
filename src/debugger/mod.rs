pub struct Debugger {
    window: minifb::Window,
    // TODO font, we can use a bitmap for this.
    buffer: Vec<u32>,
    // TODO memory page? We want to peek / display memory values.

}
