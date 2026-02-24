use iced::widget::svg;
use std::sync::LazyLock;

pub(crate) static ON_OFF: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("../assets/svgs/onoff.svg")));
pub(crate) static SLIDERS: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("../assets/svgs/sliders.svg")));
pub(crate) static ARROW_ROTATE_RIGHT: LazyLock<svg::Handle> = LazyLock::new(|| {
    svg::Handle::from_memory(include_bytes!("../assets/svgs/arrow-rotate-right.svg"))
});

pub(crate) static INSTALLED: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("../assets/svgs/installed.svg")));

pub(crate) static ON_SERVER: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("../assets/svgs/on-server.svg")));

pub(crate) static LINUX: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("../assets/svgs/linux.svg")));

pub(crate) static WINDOWS: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("../assets/svgs/windows.svg")));

pub(crate) static ARROW_LEFT: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("../assets/svgs/arrow-left.svg")));

pub(crate) static FLOPPY_DISK: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(include_bytes!("../assets/svgs/floppy-disk.svg")));
