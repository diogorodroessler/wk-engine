use winit::{error::OsError, event_loop::EventLoop, platform::wayland::WindowAttributesExtWayland,
    window::{CursorIcon, Icon, Window, WindowAttributes, WindowButtons}
};

///```Creating window```
pub struct WkWindow;
impl WkWindow
{
    ///# Create event loop for Window in Winit
    /// - ```Obs: That function is private FOR NOW```
    fn create_event_loop_with_winit() -> Result<EventLoop<()>, winit::error::EventLoopError>
    {
        let event_loop = winit::event_loop::EventLoop::new()?;

        Ok(event_loop)
    }

    ///# Creating window using the Winit
    /// `Obs: 'names_window' is names_window: (&str, &str)`
    ///     - ```Where names_window: ([0] &str = general, [1] &str = instance),```
    pub fn create_window_with_winit(
        names_window: (&str, &str), title_window: &str,
        cursor_icon: CursorIcon, is_maximized: bool,
        window_icon: Option<Icon>, is_decoration: bool,
        window_level: winit::window::WindowLevel,
        enable_buttons: WindowButtons, is_visible: bool
    ) -> Result<Window, OsError>
    {
        Self::create_event_loop_with_winit().unwrap();

        let window_attrb = WindowAttributes::default();
        let window = Self::create_event_loop_with_winit().unwrap().create_window(
            window_attrb.with_name(names_window.0, names_window.1)
            .with_title(title_window)
            .with_cursor(cursor_icon)
            .with_maximized(is_maximized)
            .with_window_icon(window_icon)
            .with_decorations(is_decoration)
            .with_window_level(window_level)
            .with_enabled_buttons(enable_buttons)
            .with_visible(is_visible)
        )?;

        Ok(window)
    }
}