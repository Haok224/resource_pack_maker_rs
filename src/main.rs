use fltk::{
    button::{Button, RoundButton},
    dialog::NativeFileChooser,
    enums::Font,
    image::PngImage,
    input::Input,
    prelude::{ButtonExt, InputExt, WidgetExt, WindowExt},
    window::Window,
    *,
};
use resource_pack_maker::reg_tool;

use fltk_theme::WidgetTheme;
use winapi::um::winreg::HKEY_CURRENT_USER;
mod ui;
fn main() {
    //app
    let app = app::App::default().with_scheme(app::Scheme::Oxy);
    let mut ui = ui::UserInterface::make_window();
    //Set theme
    let logo = image::PngImage::from_data(&[
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 16, 0, 0, 0, 16, 8,
        3, 0, 0, 0, 40, 45, 15, 83, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233, 0, 0, 0, 4,
        103, 65, 77, 65, 0, 0, 177, 143, 11, 252, 97, 5, 0, 0, 0, 123, 80, 76, 84, 69, 116, 180,
        74, 118, 182, 76, 115, 179, 73, 102, 166, 60, 111, 175, 69, 95, 159, 53, 108, 172, 66, 126,
        190, 84, 106, 170, 64, 103, 167, 61, 105, 169, 63, 97, 161, 55, 80, 144, 38, 109, 173, 67,
        117, 181, 75, 138, 185, 90, 129, 176, 81, 131, 178, 83, 89, 61, 41, 104, 168, 62, 98, 162,
        56, 147, 194, 99, 144, 191, 96, 107, 171, 65, 141, 188, 93, 156, 203, 108, 100, 164, 58,
        112, 176, 70, 127, 191, 85, 146, 193, 98, 151, 198, 103, 87, 151, 45, 96, 160, 54, 108,
        108, 108, 113, 177, 71, 121, 85, 58, 150, 108, 74, 185, 133, 92, 76, 255, 0, 135, 135, 135,
        116, 88, 68, 106, 153, 15, 198, 0, 0, 0, 9, 112, 72, 89, 115, 0, 0, 14, 195, 0, 0, 14, 195,
        1, 199, 111, 168, 100, 0, 0, 0, 170, 73, 68, 65, 84, 40, 83, 45, 142, 217, 118, 194, 64,
        12, 67, 205, 146, 20, 18, 150, 162, 178, 181, 180, 64, 144, 237, 240, 255, 95, 88, 153,
        195, 60, 204, 216, 58, 186, 210, 152, 77, 166, 179, 217, 188, 105, 63, 38, 139, 101, 215,
        175, 214, 182, 105, 183, 159, 59, 124, 237, 155, 195, 113, 218, 183, 203, 147, 157, 241,
        253, 211, 225, 2, 251, 253, 187, 226, 118, 135, 97, 0, 30, 0, 26, 172, 65, 128, 230, 244,
        128, 83, 51, 6, 186, 4, 194, 157, 89, 135, 32, 25, 22, 186, 199, 100, 4, 51, 40, 187, 73,
        12, 79, 202, 157, 2, 221, 149, 33, 253, 133, 104, 18, 101, 21, 196, 132, 60, 229, 45, 196,
        245, 102, 100, 134, 202, 124, 172, 90, 129, 73, 47, 68, 245, 168, 90, 109, 234, 82, 111,
        65, 133, 240, 153, 250, 11, 164, 196, 171, 5, 161, 112, 84, 224, 187, 86, 50, 71, 141, 181,
        18, 255, 182, 49, 29, 153, 23, 19, 2, 226, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
    ])
    .unwrap();
    //check the system theme
    let hkey = reg_tool::reg_open(
        HKEY_CURRENT_USER,
        "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
    )
    .unwrap();
    let binding = reg_tool::reg_query_binary(&hkey, "AppsUseLightTheme");
    let dark_value = binding.get(0);
    let dark_value = dark_value.unwrap(); // 0:dark 1:light
    if dark_value == &1 {
        let theme = WidgetTheme::new(fltk_theme::ThemeType::Metro);
        theme.apply();
    } else {
        let theme = WidgetTheme::new(fltk_theme::ThemeType::Dark);
        theme.apply();
    }
    //window
    println!("{dark_value}");
    let mut window: Window = ui.window;
    window.set_icon(Some(logo));
    window.show();
    //pack name input
    let mut pack_name_input = ui.pack_name;
    pack_name_input.set_callback(|c| {
        //TODO set config
        println!("{}", c.value())
    });
    let mut des_input: Input = ui.des_input;
    des_input.set_callback(|i| {
        //TODO set des config
        println!("{}", i.value())
    });
    //choose pack icon
    let mut choose_icon_button: Button = ui.icon_browse;
    let mut icon_box = ui.icon_box.clone();
    icon_box.set_frame(enums::FrameType::EngravedBox);
    let mut icon_input = ui.icon_choose;
    choose_icon_button.set_callback(move |_| {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        dialog.set_filter("*.png");
        dialog.show();
        let path = dialog.filename();
        if !path.exists() {
            return;
        }
        println!("{}", &path.to_str().unwrap_or_else(|| { "Path is none." }));
        let image = PngImage::load(path.clone()).expect("Failed to load image.");
        println!("{:?}", image);
        icon_box.set_image_scaled(Some(image));
        icon_box.redraw();
        icon_input.set_value(path.to_str().unwrap())
    });

    //Radio button
    let mut choose_ttf_mode: RoundButton = ui.choose_ttf_mode.clone();
    let choose_system_mode: RoundButton = ui.choose_system_font_mode.clone();
    let mut ttf_browse = ui.ttf_browse.clone();
    let mut b = choose_system_mode.clone();
    let mut choise = ui.system_fonts_choise.clone();
    let mut ttf_browse_a = ttf_browse.clone();
    let mut ttf_file_output = ui.ttf_output.clone();
    choose_ttf_mode.set_callback(move |sel| {
        // if other radio button is select;set its value
        if sel.value() {
            b.set_value(false);
            let _ = &choise.hide();
            ttf_browse_a.show();
            ttf_file_output.show();
        }
    });
    let mut output = ui.ttf_output.clone();
    let mut choise = ui.system_fonts_choise;
    ui.choose_system_font_mode.set_callback(move |sel| {
        if sel.value() {
            let _ = &mut choose_ttf_mode.set_value(false);
            //hide other widgets
            output.hide();
            let mut b = ui.ttf_browse.clone();
            b.hide();
            choise.show();
        }
    });
    //font set & view
    let mut ttf_out = ui.ttf_output.clone();
    let _ = &mut ttf_browse.set_callback(move |_| {
        let mut dialog = NativeFileChooser::new(dialog::FileDialogType::BrowseFile);
        dialog.set_filter("*.ttf");
        dialog.show();
        let path: std::path::PathBuf = dialog.filename();
        if !path.exists() {
            return;
        }

        let f = Font::load_font(path.clone()).unwrap();
        println!("{f}");
        // let f = Font::by_name(f.as_str());
        // println!("{}",f.get_name());
        Font::set_font(Font::Helvetica, f.as_str());
        ttf_out.set_value(path.to_str().unwrap());
    });
    //Run app
    ui.pack_config_tab.show();
    app.run().unwrap();
}
