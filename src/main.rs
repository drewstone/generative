use nannou::{
    prelude::*,
    rand::{prelude::IteratorRandom, thread_rng},
};

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let colors = vec![
        ALICEBLUE,
        ANTIQUEWHITE,
        AQUA,
        AQUAMARINE,
        AZURE,
        BEIGE,
        BISQUE,
        BLACK,
        BLANCHEDALMOND,
        BLUE,
        BLUEVIOLET,
        BROWN,
        BURLYWOOD,
        CADETBLUE,
        CHARTREUSE,
        CHOCOLATE,
        CORAL,
        CORNFLOWERBLUE,
        CORNSILK,
        CRIMSON,
        CYAN,
        DARKBLUE,
        DARKCYAN,
        DARKGOLDENROD,
        DARKGRAY,
        DARKGREY,
        DARKKHAKI,
        DARKMAGENTA,
        DARKOLIVEGREEN,
        DARKORANGE,
        DARKORCHID,
        DARKRED,
        DARKSALMON,
        DARKSEAGREEN,
        DARKSLATEBLUE,
        DARKSLATEGRAY,
        DARKSLATEGREY,
        DARKTURQUOISE,
        DARKVIOLET,
        DEEPPINK,
        DEEPSKYBLUE,
        DIMGRAY,
        DIMGREY,
        DODGERBLUE,
        FIREBRICK,
        FLORALWHITE,
        FORESTGREEN,
        FUCHSIA,
        GAINSBORO,
        GHOSTWHITE,
        GOLD,
        GOLDENROD,
        GRAY,
        GREY,
        GREEN,
        GREENYELLOW,
        HONEYDEW,
        HOTPINK,
        INDIANRED,
        INDIGO,
        IVORY,
        KHAKI,
        LAVENDER,
        LAVENDERBLUSH,
        LAWNGREEN,
        LEMONCHIFFON,
        LIGHTBLUE,
        LIGHTCORAL,
        LIGHTCYAN,
        LIGHTGOLDENRODYELLOW,
        LIGHTGRAY,
        LIGHTGREEN,
        LIGHTGREY,
        LIGHTPINK,
        LIGHTSALMON,
        LIGHTSEAGREEN,
        LIGHTSKYBLUE,
        LIGHTSLATEGRAY,
        LIGHTSLATEGREY,
        LIGHTSTEELBLUE,
        LIGHTYELLOW,
        LIME,
        LIMEGREEN,
        LINEN,
        MAGENTA,
        MAROON,
        MEDIUMAQUAMARINE,
        MEDIUMBLUE,
        MEDIUMORCHID,
        MEDIUMPURPLE,
        MEDIUMSEAGREEN,
        MEDIUMSLATEBLUE,
        MEDIUMSPRINGGREEN,
        MEDIUMTURQUOISE,
        MEDIUMVIOLETRED,
        MIDNIGHTBLUE,
        MINTCREAM,
        MISTYROSE,
        MOCCASIN,
        NAVAJOWHITE,
        NAVY,
        OLDLACE,
        OLIVE,
        OLIVEDRAB,
        ORANGE,
        ORANGERED,
        ORCHID,
        PALEGOLDENROD,
        PALEGREEN,
        PALETURQUOISE,
        PALEVIOLETRED,
        PAPAYAWHIP,
        PEACHPUFF,
        PERU,
        PINK,
        PLUM,
        POWDERBLUE,
        PURPLE,
        REBECCAPURPLE,
        RED,
        ROSYBROWN,
        ROYALBLUE,
        SADDLEBROWN,
        SALMON,
        SANDYBROWN,
        SEAGREEN,
        SEASHELL,
        SIENNA,
        SILVER,
        SKYBLUE,
        SLATEBLUE,
        SLATEGRAY,
        SLATEGREY,
        SNOW,
        SPRINGGREEN,
        STEELBLUE,
        TAN,
        TEAL,
        THISTLE,
        TOMATO,
        TURQUOISE,
        VIOLET,
        WHEAT,
        WHITE,
        WHITESMOKE,
        YELLOW,
        YELLOWGREEN,
    ];

    // Begin drawing
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(BLACK);
    let mut rng = thread_rng();

    let n_polygons = 10;
    for k in 0..n_polygons {
        // Create an `ngon` of points.
        let n_points = 2 + (1 * k);
        let radius = win.w().min(win.h()) * 0.20;
        let mut points = (0..n_points)
            .map(|i| {
                let fract = i as f32 / n_points as f32;
                let phase = fract;
                let x = (radius * (1f32 + (k as f32 / n_polygons as f32))) * (TAU * phase).cos();
                let y = (radius * (1f32 + (k as f32 / n_polygons as f32))) * (TAU * phase).sin();
                let r = fract + ((i % 3) as f32 * (1f32 + (k as f32 / n_polygons as f32)));
                let g = 1.0 - fract * (1f32 + (k as f32 / n_polygons as f32));
                let b = ((1f32 + (k as f32 / n_polygons as f32)) * fract) % 1.0;
                (pt2(x, y), rgb(r, g, b))
            })
            .collect::<Vec<_>>();
        points.push(points[0].clone());
        let rand_index = (0..colors.len()).choose(&mut rng).unwrap();
        draw.polyline()
            .x(win.w() * 0.40)
            .rotate(-t * 0.1)
            .color(colors[rand_index])
            .weight(1.0)
            .join_round()
            .points_colored(points.clone());
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
