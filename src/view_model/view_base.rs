pub struct View {
    _width: isize,
    _height: isize,
}

pub trait ViewRenderer {
    fn display();
}