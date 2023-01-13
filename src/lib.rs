// C
use libc::c_char;
use std::ffi::CString;

// Other

// html
use visdom::Vis;

#[no_mangle]
pub unsafe extern "C" fn add_spaces(html_str: *const c_char) -> *const c_char {
    let root = Vis::load(html_str.as_ref().unwrap().to_string()).unwrap();
    //println!("Html file: {}", html_str.as_ref().unwrap().to_string());

    let mut text_lines = root.find("p");

    // Be scared
    let font_size: usize = str::parse(
        text_lines
            .children("span")
            .attr("style")
            .unwrap()
            .to_string()
            .split("font-size:")
            .last()
            .unwrap()
            .to_string()
            .split('.')
            .next()
            .unwrap(),
    )
    .unwrap();
    println!("Font size: {}", font_size);
    let text_lines_size = text_lines.length();
    println!("Items count: {:?}", text_lines_size);
    let mut previous_cord: f32 = 0.0;
    // The problem here is that it iterates over span, it could be a problem in the future
    text_lines.for_each(|index, element| {
        //println!("Element {}: \n {}", index, element.text());
        // If line 2 top position - 1 line top position > font size * 2 then add <br>
        // Get top cords

        // get_attribute is nowhere documented ;)...
        let top_cord: f32 = str::parse(
            &element
                .get_attribute("style")
                .unwrap()
                .to_string()
                .split(';')
                .next()
                .unwrap()
                .to_string()
                .replace("top:", "")
                .replace("pt", ""),
        )
        .unwrap();
        //println!("Top cord {}", top_cord);
        if index == 0 {
            previous_cord = top_cord;
        } else {
            if top_cord - previous_cord >= str::parse::<f32>(&font_size.to_string()).unwrap() * 2.0
            {
                let mut tmp_str = element.html();
                tmp_str = format!("{}{}", "<br>", tmp_str);
                //println!("Added enter \n{}", tmp_str);
                element.set_html(&tmp_str);
            }
            previous_cord = top_cord;
        }

        index <= text_lines_size
    });

    // logs
    println!("Output is: ");
    text_lines.for_each(|index, element| {
        println!("{}", element.html());

        index <= text_lines_size
    });

    println!("Result is: \n{}", root.html());

    // What is this warning
    return CString::new(root.html()).unwrap().as_ptr();
}
