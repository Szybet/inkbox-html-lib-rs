let start_html_str = ">"; // Before < to mark where it begins
let stop_html_str = "</"; // After </ to mark where something ends
let start_pure_high = "<b>";
let stop_pure_high = "</b>";

let start_highlight_str = format!("{}{}", start_html_str, start_pure_high);
let stop_highlight_str = format!("{}{}", stop_pure_high, stop_html_str);

let mut highlight = html;
if highlight.ends_with("\n") {
    let _ = highlight.pop();
}
highlight = highlight + &stop_pure_high.to_owned();

highlight = highlight.replace(start_html_str, &start_highlight_str);
highlight = highlight.replace(stop_html_str, &stop_highlight_str);

// Normal
    /*

let combined = format!("{}{}", start_pure_high, stop_pure_high).to_string();
highlight = highlight.replace(&combined, "");

// Normal reverse
let combined_reverse = format!("{}{}", stop_pure_high, start_pure_high).to_string();
highlight = highlight.replace(&combined_reverse, "");

// Stop not Normal
let stop_bug = format!("{}{}", stop_pure_high, stop_pure_high).to_string();
highlight = highlight.replace(&stop_bug, "");

let start_bug = format!("{}{}", start_pure_high, start_pure_high).to_string();
highlight = highlight.replace(&start_bug, "");
 */

// Do this at the end for weird reasons
highlight = format!("{}{}", start_pure_high, highlight);

println!("highlight html code finished: {}", highlight);

highlight
